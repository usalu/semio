//! Standalone scratch verification of the deflate rfc1950 container header math (CMF/FLG/FCHECK/
//! DICTID/Adler32) and the diff/mutation algebra (apply/absorb/inverse/between), independent of
//! the full workspace build. Mirrors the real implementation in
//! ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/... exactly (same formulas, same field shapes).

//#region Adler32 (copied verbatim from the real engine -- untouched codec logic)
fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}
//#endregion

//#region Minimal raw deflate stand-in (stored blocks only -- real engine uses LZ77+Huffman;
// irrelevant to verifying the CONTAINER math, which is what this scratch checks)
fn deflate_raw_stored(data: &[u8]) -> Vec<u8> {
    // BFINAL=1 BTYPE=00 (stored), byte-aligned LEN/NLEN + raw bytes.
    let mut out = vec![0x01u8]; // bit0=BFINAL=1, bits1-2=00 stored, rest padding zero -> byte 0x01
    let len = data.len() as u16;
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&(!len).to_le_bytes());
    out.extend_from_slice(data);
    out
}
fn inflate_raw_stored(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() { return Err("empty".into()); }
    let bfinal = data[0] & 1;
    let btype = (data[0] >> 1) & 0b11;
    assert_eq!(bfinal, 1);
    assert_eq!(btype, 0);
    let len = u16::from_le_bytes([data[1], data[2]]) as usize;
    let nlen = u16::from_le_bytes([data[3], data[4]]);
    if (len as u16) ^ 0xFFFF != nlen { return Err("LEN/NLEN mismatch".into()); }
    Ok(data[5..5 + len].to_vec())
}
//#endregion

//#region DeflateLevelHint
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeflateLevelHint { Fastest, Fast, Default, Maximum }
impl Default for DeflateLevelHint { fn default() -> Self { DeflateLevelHint::Default } }
impl DeflateLevelHint {
    fn from_bits(bits: u8) -> Self {
        match bits & 0b11 { 0 => Self::Fastest, 1 => Self::Fast, 2 => Self::Default, _ => Self::Maximum }
    }
    fn to_bits(self) -> u8 {
        match self { Self::Fastest => 0, Self::Fast => 1, Self::Default => 2, Self::Maximum => 3 }
    }
}
//#endregion

//#region DeflateSnapshot
#[derive(Clone, Debug, PartialEq)]
struct DeflateSnapshot {
    schema: String,
    compression_method: u8,
    window_bits: u8,
    compression_level_hint: DeflateLevelHint,
    dict_id: Option<u32>,
    payload: Vec<u8>,
}
impl Default for DeflateSnapshot {
    fn default() -> Self {
        Self { schema: "stdio.deflate".into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::default(), dict_id: None, payload: Vec::new() }
    }
}
//#endregion

//#region Engine: encode/decode snapshot <-> zlib bytes (mirrors real engine exactly)
fn encode_deflate_snapshot(snapshot: &DeflateSnapshot) -> Vec<u8> {
    let cmf = ((snapshot.window_bits & 0x0F) << 4) | (snapshot.compression_method & 0x0F);
    let fdict = snapshot.dict_id.is_some();
    let flg_hi = (snapshot.compression_level_hint.to_bits() << 6) | ((fdict as u8) << 5);
    let fcheck = (31 - (((cmf as u16) * 256 + flg_hi as u16) % 31)) % 31;
    let flg = flg_hi | (fcheck as u8);

    let raw = deflate_raw_stored(&snapshot.payload);
    let mut out = Vec::with_capacity(2 + 4 + raw.len() + 4);
    out.push(cmf);
    out.push(flg);
    if let Some(dict_id) = snapshot.dict_id {
        out.extend_from_slice(&dict_id.to_be_bytes());
    }
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(&snapshot.payload).to_be_bytes());
    out
}

fn decode_deflate_snapshot(data: &[u8]) -> Result<DeflateSnapshot, String> {
    if data.len() < 6 { return Err("zlib stream too short".into()); }
    let cmf = data[0];
    let flg = data[1];
    let compression_method = cmf & 0x0F;
    let window_bits = (cmf >> 4) & 0x0F;
    if compression_method != 8 { return Err("unsupported zlib compression method".into()); }
    if ((cmf as u16) * 256 + flg as u16) % 31 != 0 { return Err("zlib CMF/FLG check failed".into()); }
    let fdict = flg & 0x20 != 0;
    let compression_level_hint = DeflateLevelHint::from_bits(flg >> 6);

    let mut pos = 2usize;
    let dict_id = if fdict {
        if data.len() < pos + 4 { return Err("truncated preset dictionary id".into()); }
        let id = u32::from_be_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]);
        pos += 4;
        Some(id)
    } else { None };

    if data.len() < pos + 4 { return Err("zlib stream too short".into()); }
    let adler_bytes = &data[data.len()-4..];
    let expect = u32::from_be_bytes([adler_bytes[0], adler_bytes[1], adler_bytes[2], adler_bytes[3]]);
    let raw = &data[pos..data.len()-4];
    let payload = inflate_raw_stored(raw)?;
    let got = adler32(&payload);
    if got != expect { return Err(format!("adler32 mismatch: expected {expect:#010x}, got {got:#010x}")); }

    Ok(DeflateSnapshot { schema: "stdio.deflate".into(), compression_method, window_bits, compression_level_hint, dict_id, payload })
}
//#endregion

//#region DeflateDiff + algebra (mirrors real diff/component.rs exactly)
#[derive(Clone, Debug, Default, PartialEq)]
struct DeflateDiff {
    compression_method: Option<u8>,
    window_bits: Option<u8>,
    compression_level_hint: Option<DeflateLevelHint>,
    dict_id: Option<Option<u32>>,
    payload: Option<Vec<u8>>,
}
impl DeflateDiff {
    fn apply(&self, base: &DeflateSnapshot) -> DeflateSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.compression_method { next.compression_method = v; }
        if let Some(v) = self.window_bits { next.window_bits = v; }
        if let Some(v) = self.compression_level_hint { next.compression_level_hint = v; }
        if let Some(v) = self.dict_id { next.dict_id = v; }
        if let Some(v) = &self.payload { next.payload = v.clone(); }
        next
    }
    fn absorb(&mut self, other: Self) {
        if other.compression_method.is_some() { self.compression_method = other.compression_method; }
        if other.window_bits.is_some() { self.window_bits = other.window_bits; }
        if other.compression_level_hint.is_some() { self.compression_level_hint = other.compression_level_hint; }
        if other.dict_id.is_some() { self.dict_id = other.dict_id; }
        if other.payload.is_some() { self.payload = other.payload; }
    }
    fn inverse(&self, base: &DeflateSnapshot) -> Self {
        DeflateDiff {
            compression_method: self.compression_method.map(|_| base.compression_method),
            window_bits: self.window_bits.map(|_| base.window_bits),
            compression_level_hint: self.compression_level_hint.map(|_| base.compression_level_hint),
            dict_id: self.dict_id.map(|_| base.dict_id),
            payload: self.payload.as_ref().map(|_| base.payload.clone()),
        }
    }
    fn between(base: &DeflateSnapshot, other: &DeflateSnapshot) -> Self {
        DeflateDiff {
            compression_method: (base.compression_method != other.compression_method).then_some(other.compression_method),
            window_bits: (base.window_bits != other.window_bits).then_some(other.window_bits),
            compression_level_hint: (base.compression_level_hint != other.compression_level_hint).then_some(other.compression_level_hint),
            dict_id: (base.dict_id != other.dict_id).then_some(other.dict_id),
            payload: (base.payload != other.payload).then_some(other.payload.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.compression_method.is_none() && self.window_bits.is_none() && self.compression_level_hint.is_none() && self.dict_id.is_none() && self.payload.is_none()
    }
}
//#endregion

fn main() {
    // 🌱 REAL_FIXTURE_ZLIB: identical bytes to 🗜️example.zz / 🗣️example.dsl.semio (fixed).
    const REAL_FIXTURE_ZLIB: &[u8] = &[
        0x78, 0x9c, 0x2b, 0x2e, 0x49, 0xc9, 0xcc, 0xd7, 0x4b, 0x49, 0x4d, 0xcb, 0x49, 0x2c, 0x49,
        0x55, 0x48, 0xce, 0xcf, 0x4b, 0xcb, 0x2f, 0xca, 0x4d, 0xcc, 0x4b, 0x4e, 0x55, 0x48, 0xcb,
        0xac, 0x28, 0x29, 0x2d, 0x4a, 0x05, 0x00, 0xda, 0xb1, 0x0c, 0xf9,
    ];
    // NOTE: this fixture uses DYNAMIC Huffman (not our stored-block stand-in), so this scratch's
    // toy `inflate_raw_stored` can't decode it -- that's fine, it's only exercised by the real
    // Huffman-capable engine in the workspace crate. Here we just verify the CMF/FLG/adler
    // *header* parsing math against it directly (bypassing inflate), and do full round trips
    // with our own toy codec + typed fields.
    let cmf = REAL_FIXTURE_ZLIB[0];
    let flg = REAL_FIXTURE_ZLIB[1];
    assert_eq!(cmf & 0x0F, 8, "compression method");
    assert_eq!((cmf >> 4) & 0x0F, 7, "window bits");
    assert_eq!(flg & 0x20, 0, "fdict clear");
    assert_eq!((flg >> 6) & 0x03, 2, "flevel default");
    assert_eq!(((cmf as u16) * 256 + flg as u16) % 31, 0, "check bits");
    println!("[ok] real fixture CMF/FLG header math verified");

    // Self round trip with our toy stored-block codec.
    let snap = DeflateSnapshot {
        schema: "stdio.deflate".into(),
        compression_method: 8,
        window_bits: 5,
        compression_level_hint: DeflateLevelHint::Maximum,
        dict_id: Some(0x1234_5678),
        payload: b"preset-dictionary-id-round-trip".to_vec(),
    };
    let bytes = encode_deflate_snapshot(&snap);
    assert_eq!(bytes[1] & 0x20, 0x20, "fdict set");
    let decoded = decode_deflate_snapshot(&bytes).expect("decode");
    assert_eq!(decoded, snap);
    println!("[ok] self round trip with preset dictionary");

    // FCHECK corruption must be rejected.
    let mut bad = encode_deflate_snapshot(&DeflateSnapshot::default());
    bad[1] ^= 0x01;
    assert!(decode_deflate_snapshot(&bad).is_err());
    println!("[ok] corrupted FCHECK rejected");

    // field_sweep
    let a = DeflateSnapshot { schema: "stdio.deflate".into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Fastest, dict_id: None, payload: b"sweep-a-payload".to_vec() };
    let b = DeflateSnapshot { schema: "stdio.deflate".into(), compression_method: 9, window_bits: 6, compression_level_hint: DeflateLevelHint::Maximum, dict_id: Some(0xDEAD_BEEF), payload: b"sweep-b-different-longer-payload".to_vec() };
    let ab = DeflateDiff::between(&a, &b);
    assert!(ab.compression_method.is_some() && ab.window_bits.is_some() && ab.compression_level_hint.is_some() && ab.dict_id.is_some() && ab.payload.is_some());
    assert_eq!(ab.dict_id, Some(Some(0xDEAD_BEEF)));
    assert_eq!(ab.apply(&a), b);
    let ba = DeflateDiff::between(&b, &a);
    assert_eq!(ba.dict_id, Some(None));
    assert_eq!(ba.apply(&b), a);
    assert!(DeflateDiff::between(&a, &a).is_empty());
    println!("[ok] field_sweep between() covers every field, tri-state Some(None) exercised");

    // inverse_law (diff-level)
    let d = DeflateDiff::between(&a, &b);
    let applied = d.apply(&a);
    let undone = d.inverse(&a).apply(&applied);
    assert_eq!(undone, a);
    println!("[ok] diff-level inverse_law");

    // absorb_law: disjoint + LWW + associativity
    let base = a.clone();
    let d1 = DeflateDiff { compression_method: Some(8), window_bits: Some(5), compression_level_hint: Some(DeflateLevelHint::Fast), ..Default::default() };
    let d2 = DeflateDiff { payload: Some(b"absorbed-payload".to_vec()), ..Default::default() };
    let mut absorbed = d1.clone();
    absorbed.absorb(d2.clone());
    assert_eq!(absorbed.apply(&base), d2.apply(&d1.apply(&base)));

    let d3 = DeflateDiff { payload: Some(b"first".to_vec()), ..Default::default() };
    let d4 = DeflateDiff { payload: Some(b"second".to_vec()), ..Default::default() };
    let mut lww = d3.clone();
    lww.absorb(d4.clone());
    assert_eq!(lww.payload, Some(b"second".to_vec()));

    let da = DeflateDiff { compression_method: Some(9), window_bits: Some(6), compression_level_hint: Some(DeflateLevelHint::Maximum), ..Default::default() };
    let db = DeflateDiff { dict_id: Some(Some(11)), ..Default::default() };
    let dc = DeflateDiff { payload: Some(b"triple".to_vec()), ..Default::default() };
    let mut left = da.clone(); left.absorb(db.clone()); left.absorb(dc.clone());
    let mut right_tail = db.clone(); right_tail.absorb(dc.clone());
    let mut right = da.clone(); right.absorb(right_tail);
    assert_eq!(left, right);
    println!("[ok] absorb_law: disjoint composition, LWW, associativity");

    println!("ALL SCRATCH CHECKS PASSED");
}
