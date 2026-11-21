use semio_plugin_interpreter::{CoreError, OwnedSemioArtifact};
use serde_json::Value;

fn read_number(bytes: &mut &[u8]) -> usize {
    let mut value = 0;
    for shift in (0..35).step_by(7) {
        let byte = bytes[0];
        *bytes = &bytes[1..];
        value |= usize::from(byte & 127) << shift;
        if byte < 128 { return value }
    }
    panic!("bad fixture integer");
}

fn number(mut value: usize, output: &mut Vec<u8>) {
    while value >= 128 {
        output.push((value & 127) as u8 | 128);
        value >>= 7;
    }
    output.push(value as u8);
}

fn section(tag: u8, bytes: &[u8], output: &mut Vec<u8>) {
    output.push(tag);
    number(bytes.len(), output);
    output.extend_from_slice(bytes);
}

fn sections(mut module: &[u8]) -> Vec<(u8, &[u8])> {
    module = &module[8..];
    let mut output = Vec::new();
    while let Some((&tag, rest)) = module.split_first() {
        module = rest;
        let length = read_number(&mut module);
        output.push((tag, &module[..length]));
        module = &module[length..];
    }
    output
}

fn bytes(value: &Value) -> Vec<u8> {
    let hex = value.as_str().unwrap();
    (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()).collect()
}

fn actual_owned_component() -> (Vec<u8>, [u8; 32], Value) {
    let owned: Value = serde_json::from_str(include_str!(env!("SEMIO_PREPARATION_FIXTURE"))).unwrap();
    let row: Value = serde_json::from_str(include_str!(env!("SEMIO_CORE_IMAGE_FIXTURE"))).unwrap();
    let owned = bytes(&owned["coreHex"]);
    let core = bytes(&row["coreHex"]);
    let owned = sections(&owned);
    let mut module = b"\0asm\x01\0\0\0".to_vec();
    for (tag, small) in sections(&core) {
        let mut graft = owned.iter().find(|entry| entry.0 == tag).map_or(&[][..], |entry| entry.1);
        let mut payload = Vec::new();
        match tag {
            1 => {
                let count = read_number(&mut graft);
                number(count + 1, &mut payload);
                payload.extend_from_slice(&small[1..]);
                payload.extend_from_slice(graft);
            }
            3 => {
                number(37, &mut payload);
                payload.push(0);
                read_number(&mut graft);
                payload.extend(graft[..36].iter().map(|ty| ty + 1));
            }
            7 => {
                number(42, &mut payload);
                payload.extend_from_slice(&small[1..]);
                read_number(&mut graft);
                while !graft.is_empty() {
                    let name_length = read_number(&mut graft);
                    let name = &graft[..name_length];
                    let kind = graft[name_length];
                    graft = &graft[name_length + 1..];
                    let index = read_number(&mut graft);
                    if kind != 0 || index >= 36 { continue }
                    number(name_length, &mut payload);
                    payload.extend(name);
                    payload.push(kind);
                    number(index + 1, &mut payload);
                }
            }
            10 => {
                number(37, &mut payload);
                payload.extend_from_slice(&small[1..]);
                read_number(&mut graft);
                for _ in 0..36 {
                    let length = read_number(&mut graft);
                    number(length, &mut payload);
                    payload.extend_from_slice(&graft[..length]);
                    graft = &graft[length..];
                }
            }
            _ => payload.extend_from_slice(small),
        }
        section(tag, &payload, &mut module);
    }
    let mut component = b"\0asm\x0d\0\x01\0".to_vec();
    section(1, &module, &mut component);
    let digest = bytes(&Value::String(semio_framework_hash::sha256_hex(&component))).try_into().unwrap();
    (component, digest, row)
}

#[test]
fn owned_sealed_image_round_trips_exact_portable_payload() {
    let (component, digest, row) = actual_owned_component();
    let first = OwnedSemioArtifact::parse(&component).unwrap();
    let fresh = OwnedSemioArtifact::parse(&component).unwrap();
    let prepared = first.prepare_actor(128, |_| true).unwrap().unwrap();
    let mut wire = Vec::new();
    prepared.write_image(digest, 1 << 20, |chunk| { wire.extend_from_slice(chunk); Ok(()) }).unwrap();
    assert_eq!(wire.len(), row["wireBytes"].as_u64().unwrap() as usize);
    assert_eq!(&wire[..9], &bytes(&row["prefix"]));
    assert_eq!(&wire[9..41], &digest);
    assert_eq!(&wire[65..68], &[10, 93, 30]);
    assert!(wire[68..65 + 65_536].iter().all(|byte| *byte == 0));
    let table = 65 + 65_536 + 16;
    assert_eq!(&wire[table..table + 24], &[0u64, 0, u64::MAX].into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());
    let globals = table + 24 + 8;
    assert_eq!(&wire[globals..globals + 12], &bytes(&row["globalWire"]));
    assert_eq!(&wire[wire.len() - 3..], &[0, 1, 0]);
    assert_eq!(&wire[wire.len() - 14..wire.len() - 11], &[0, 1, 0]);
    let mut neutral = wire.clone();
    neutral[9..41].copy_from_slice(&bytes(&row["componentSha256"]));
    let neutral_fingerprint = bytes(&row["coreHex"]).into_iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    neutral[41..49].copy_from_slice(&neutral_fingerprint.to_le_bytes());
    assert_eq!(semio_framework_hash::sha256_hex(&neutral), row["imageSha256"].as_str().unwrap());
    let restored = fresh.restore_actor_preparation(digest, &wire, wire.len(), 1 << 20, || true).unwrap();
    let mut again = Vec::new();
    restored.write_image(digest, wire.len(), |chunk| { again.extend_from_slice(chunk); Ok(()) }).unwrap();
    assert_eq!(again, wire);

    for (offset, value) in [(8, 0), (40, wire[40] ^ 0x80), (49, 2), (58, 1), (table, 0xff)] {
        let mut forged = wire.clone();
        forged[offset] = value;
        assert!(fresh.restore_actor_preparation(digest, &forged, wire.len(), 1 << 20, || true).is_err());
    }
    assert!(fresh.restore_actor_preparation(digest, &wire, wire.len() - 1, 1 << 20, || true).is_err());
    assert!(fresh.restore_actor_preparation(digest, &wire, wire.len(), 65_535, || true).is_err());
    assert!(fresh.restore_actor_preparation(digest, &wire[..wire.len() - 1], wire.len(), 1 << 20, || true).is_err());
    let mut extra = wire.clone();
    extra.push(0);
    assert!(fresh.restore_actor_preparation(digest, &extra, extra.len(), 1 << 20, || true).is_err());
    assert!(matches!(fresh.restore_actor_preparation(digest, &wire, wire.len(), 1 << 20, || false),
        Err(CoreError::State(message)) if message.contains("cancelled")));
}
