//! 🧪️ Verifies the authored Poll frame header and row metadata against independent frames.
#[path = "instance-open-frame.rs"]
mod frame;
use frame::{ASSET_HEADER_BYTES, AssetHeader, CAPABILITY_HEADER_BYTES, CapabilityHeader, HEADER_BYTES, InvalidFrame, OpenHeader, field};
use std::io::Read;

fn take<'a>(wire: &'a [u8], cursor: &mut usize, length: usize) -> Result<&'a [u8], InvalidFrame> {
    let data = field(wire, *cursor, length)?;
    *cursor += length;
    Ok(data)
}

fn text(bytes: &[u8]) -> Result<(), InvalidFrame> {
    std::str::from_utf8(bytes).map_err(|_| InvalidFrame)?;
    Ok(())
}

fn roundtrip(wire: &[u8]) -> Result<(Vec<u8>, OpenHeader, Vec<CapabilityHeader>), InvalidFrame> {
    let mut cursor = 0;
    let header = OpenHeader::read(take(wire, &mut cursor, HEADER_BYTES)?, wire.len())?;
    let mut encoded = header.bytes(wire.len())?.to_vec();
    for extent in [header.app_bytes, header.actor_bytes] {
        let value=take(wire,&mut cursor,extent as usize)?;
        text(value)?;
        encoded.extend(value);
    }
    encoded.extend(take(wire,&mut cursor,header.config_bytes as usize)?);
    for _ in 0..header.assets {
        let asset = AssetHeader::read(take(wire, &mut cursor, ASSET_HEADER_BYTES)?)?;
        let name = take(wire, &mut cursor, asset.name_bytes as usize)?;
        let data = take(wire, &mut cursor, asset.data_bytes as usize)?;
        text(name)?;
        encoded.extend(asset.bytes());
        encoded.extend(name);
        encoded.extend(data);
    }
    let mut capabilities = Vec::new();
    for _ in 0..header.capabilities {
        let header = CapabilityHeader::read(take(wire, &mut cursor, CAPABILITY_HEADER_BYTES)?)?;
        let id = take(wire, &mut cursor, header.id_bytes as usize)?;
        let scope = take(wire, &mut cursor, header.scope_bytes as usize)?;
        text(id)?;
        text(scope)?;
        encoded.extend(header.bytes());
        encoded.extend(id);
        encoded.extend(scope);
        capabilities.push(header);
    }
    if cursor != wire.len() { return Err(InvalidFrame); }
    Ok((encoded, header, capabilities))
}

fn main() {
    let mut wire = Vec::new();
    std::io::stdin().read_to_end(&mut wire).unwrap();
    let Ok((encoded, header, capabilities)) = roundtrip(&wire) else { println!("invalid"); return };
    println!("{}", encoded.iter().map(|value|format!("{value:02x}")).collect::<String>());
    println!("{} {} {}",header.activation_generation,header.instance_id,header.request_sequence);
    for value in header.limits { print!("{value} "); }
    println!();
    for quota in header.quotas { print!("{} ",quota.map_or_else(||"-".into(),|value|value.to_string())); }
    println!();
    for capability in capabilities { println!("{} {}",capability.token,capability.expires_ms.map_or_else(||"-".into(),|value|value.to_string())); }
}
