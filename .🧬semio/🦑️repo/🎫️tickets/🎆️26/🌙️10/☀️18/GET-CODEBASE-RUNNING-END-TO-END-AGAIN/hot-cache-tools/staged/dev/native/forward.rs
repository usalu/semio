//! 🚪 semio-repo-forward: std-only reference thin client of the resident service (the native `repo` launcher embeds the same steps).
//! `semio-repo-forward --root R [--languages en,de] [--lane L] [--deadline MS] [--timing] -- <command> <args…>`
//! Exit codes: the command's own code; 75 asks the caller to run the command in-process (no service, refused with retry=fallback).
//! https://datatracker.ietf.org/doc/html/rfc2104 (HMAC), https://csrc.nist.gov/pubs/fips/180-4/upd1/final (SHA-256)
use std::io::{BufRead, BufReader, Write};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ───────────── SHA-256 / HMAC ─────────────

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut message = data.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&((data.len() as u64) * 8).to_be_bytes());
    for chunk in message.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, value) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut block = [0u8; 64];
    if key.len() > 64 {
        block[..32].copy_from_slice(&sha256(key));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let mut inner: Vec<u8> = block.iter().map(|b| b ^ 0x36).collect();
    inner.extend_from_slice(data);
    let inner_hash = sha256(&inner);
    let mut outer: Vec<u8> = block.iter().map(|b| b ^ 0x5c).collect();
    outer.extend_from_slice(&inner_hash);
    sha256(&outer)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn base64url(bytes: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = (chunk[0] as u32) << 16 | (*chunk.get(1).unwrap_or(&0) as u32) << 8 | *chunk.get(2).unwrap_or(&0) as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        if chunk.len() > 1 {
            out.push(T[(n >> 6) as usize & 63] as char);
        }
        if chunk.len() > 2 {
            out.push(T[n as usize & 63] as char);
        }
    }
    out
}

/// Mints the launcher capability: scope status|query|run|watch and a one minute lifetime (same canonical caveat strings as the JS implementation).
pub fn launcher_capability(key: &[u8], id: &str, not_after_ms: u64) -> String {
    let caveats = [r#"{"k":"scope","v":["status","query","run","watch"]}"#.to_string(), format!(r#"{{"k":"notAfter","v":{not_after_ms}}}"#)];
    let mut mac = hmac_sha256(key, id.as_bytes());
    for caveat in &caveats {
        mac = hmac_sha256(&mac, caveat.as_bytes());
    }
    let payload = format!(r#"{{"id":"{id}","caveats":[{}]}}"#, caveats.join(","));
    format!("v1.{}.{}", base64url(payload.as_bytes()), base64url(&mac))
}

// ───────────── endpoint ─────────────

pub fn worktree_id(root: &str) -> String {
    let mut text = b"semio-repo-service-v1\0".to_vec();
    text.extend_from_slice(root.as_bytes());
    hex(&sha256(&text))[..12].to_string()
}

#[cfg(unix)]
extern "C" {
    fn getuid() -> u32;
}

pub struct Runtime {
    pub directory: String,
    pub endpoint: String,
    pub key: String,
}

pub fn runtime(root: &str) -> Runtime {
    let id = worktree_id(root);
    #[cfg(unix)]
    {
        let owner = unsafe { getuid() }.to_string();
        let base = std::env::var("XDG_RUNTIME_DIR").ok().filter(|v| !v.is_empty()).unwrap_or_else(|| std::env::temp_dir().to_string_lossy().trim_end_matches('/').to_string());
        let mut directory = format!("{base}/semio-repo-{owner}/{id}");
        if directory.len() + 2 > 96 {
            directory = format!("/tmp/semio-repo-{owner}/{id}");
        }
        Runtime { endpoint: format!("{directory}/s"), key: format!("{directory}/key"), directory }
    }
    #[cfg(windows)]
    {
        let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
        let directory = format!("{base}\\semio\\repo\\{id}");
        Runtime { endpoint: format!("\\\\.\\pipe\\semio-repo--1-{id}"), key: format!("{directory}\\key"), directory }
    }
}

// ───────────── minimal JSON ─────────────

#[derive(Debug, Clone)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        if let Json::Obj(items) = self {
            items.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        } else {
            None
        }
    }
    pub fn str(&self) -> Option<&str> {
        if let Json::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn num(&self) -> Option<f64> {
        if let Json::Num(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}

pub fn parse_json(text: &str) -> Option<Json> {
    let bytes = text.as_bytes();
    let mut at = 0;
    let value = parse_value(bytes, &mut at)?;
    skip(bytes, &mut at);
    if at == bytes.len() {
        Some(value)
    } else {
        None
    }
}

fn skip(b: &[u8], at: &mut usize) {
    while *at < b.len() && (b[*at] as char).is_ascii_whitespace() {
        *at += 1;
    }
}

fn parse_value(b: &[u8], at: &mut usize) -> Option<Json> {
    skip(b, at);
    match *b.get(*at)? {
        b'n' if b[*at..].starts_with(b"null") => {
            *at += 4;
            Some(Json::Null)
        }
        b't' if b[*at..].starts_with(b"true") => {
            *at += 4;
            Some(Json::Bool(true))
        }
        b'f' if b[*at..].starts_with(b"false") => {
            *at += 5;
            Some(Json::Bool(false))
        }
        b'"' => parse_string(b, at).map(Json::Str),
        b'[' => {
            *at += 1;
            let mut items = Vec::new();
            skip(b, at);
            if b.get(*at) == Some(&b']') {
                *at += 1;
                return Some(Json::Arr(items));
            }
            loop {
                items.push(parse_value(b, at)?);
                skip(b, at);
                match *b.get(*at)? {
                    b',' => *at += 1,
                    b']' => {
                        *at += 1;
                        return Some(Json::Arr(items));
                    }
                    _ => return None,
                }
            }
        }
        b'{' => {
            *at += 1;
            let mut items = Vec::new();
            skip(b, at);
            if b.get(*at) == Some(&b'}') {
                *at += 1;
                return Some(Json::Obj(items));
            }
            loop {
                skip(b, at);
                let key = parse_string(b, at)?;
                skip(b, at);
                if *b.get(*at)? != b':' {
                    return None;
                }
                *at += 1;
                items.push((key, parse_value(b, at)?));
                skip(b, at);
                match *b.get(*at)? {
                    b',' => *at += 1,
                    b'}' => {
                        *at += 1;
                        return Some(Json::Obj(items));
                    }
                    _ => return None,
                }
            }
        }
        _ => {
            let start = *at;
            while *at < b.len() && matches!(b[*at], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                *at += 1;
            }
            std::str::from_utf8(&b[start..*at]).ok()?.parse::<f64>().ok().map(Json::Num)
        }
    }
}

fn parse_string(b: &[u8], at: &mut usize) -> Option<String> {
    if *b.get(*at)? != b'"' {
        return None;
    }
    *at += 1;
    let mut out: Vec<u8> = Vec::new();
    loop {
        let c = *b.get(*at)?;
        *at += 1;
        match c {
            b'"' => return String::from_utf8(out).ok(),
            b'\\' => {
                let e = *b.get(*at)?;
                *at += 1;
                match e {
                    b'n' => out.push(b'\n'),
                    b't' => out.push(b'\t'),
                    b'r' => out.push(b'\r'),
                    b'b' => out.push(8),
                    b'f' => out.push(12),
                    b'u' => {
                        let hex4 = std::str::from_utf8(b.get(*at..*at + 4)?).ok()?;
                        let mut code = u32::from_str_radix(hex4, 16).ok()?;
                        *at += 4;
                        if (0xD800..0xDC00).contains(&code) && b.get(*at) == Some(&b'\\') && b.get(*at + 1) == Some(&b'u') {
                            let low = u32::from_str_radix(std::str::from_utf8(b.get(*at + 2..*at + 6)?).ok()?, 16).ok()?;
                            code = 0x10000 + ((code - 0xD800) << 10) + (low.wrapping_sub(0xDC00));
                            *at += 6;
                        }
                        let ch = char::from_u32(code).unwrap_or('\u{fffd}');
                        let mut buffer = [0u8; 4];
                        out.extend_from_slice(ch.encode_utf8(&mut buffer).as_bytes());
                    }
                    other => out.push(other),
                }
            }
            other => out.push(other),
        }
    }
}

fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// ───────────── client ─────────────

#[cfg(unix)]
type Stream = std::os::unix::net::UnixStream;
#[cfg(windows)]
type Stream = std::fs::File;

#[cfg(unix)]
fn open(endpoint: &str) -> std::io::Result<Stream> {
    Stream::connect(endpoint)
}
#[cfg(windows)]
fn open(endpoint: &str) -> std::io::Result<Stream> {
    std::fs::OpenOptions::new().read(true).write(true).open(endpoint)
}

#[cfg(unix)]
fn trusted(directory: &str) -> bool {
    use std::os::unix::fs::MetadataExt;
    match std::fs::symlink_metadata(directory) {
        Ok(m) => m.is_dir() && m.uid() == unsafe { getuid() } && (m.mode() & 0o077) == 0,
        Err(_) => false,
    }
}
#[cfg(windows)]
fn trusted(_directory: &str) -> bool {
    true
}

fn main() {
    let started = Instant::now();
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.first().map(String::as_str) {
        Some("--print-capability") if arguments.len() == 4 => {
            let key: Vec<u8> = (0..arguments[1].len() / 2).map(|i| u8::from_str_radix(&arguments[1][i * 2..i * 2 + 2], 16).unwrap_or(0)).collect();
            println!("{}", launcher_capability(&key, &arguments[2], arguments[3].parse().unwrap_or(0)));
            return;
        }
        Some("--print-endpoint") if arguments.len() == 2 => {
            let r = runtime(&arguments[1]);
            println!("{}\n{}", worktree_id(&arguments[1]), r.endpoint);
            return;
        }
        _ => {}
    }
    let split = arguments.iter().position(|a| a == "--").unwrap_or(arguments.len());
    let (flags, rest) = arguments.split_at(split);
    let rest = if rest.is_empty() { rest } else { &rest[1..] };
    let flag = |name: &str, fallback: &str| flags.iter().position(|a| a == name).and_then(|at| flags.get(at + 1)).cloned().unwrap_or_else(|| fallback.to_string());
    let timing = flags.iter().any(|a| a == "--timing");
    let verbose = flags.iter().any(|a| a == "--verbose");
    let root = flag("--root", &std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default());
    let languages: Vec<String> = flag("--languages", "en").split(',').map(|s| s.to_string()).collect();
    let fallback = |reason: &str| -> ! {
        if verbose {
            eprintln!("[fallback] {reason}");
        }
        std::process::exit(75);
    };
    if rest.is_empty() {
        fallback("no command");
    }
    let runtime = runtime(&root);
    if !trusted(&runtime.directory) {
        fallback("down");
    }
    let key = match std::fs::read(&runtime.key) {
        Ok(k) => k,
        Err(_) => fallback("key"),
    };
    let mut id_bytes = [0u8; 8];
    if let Ok(mut f) = std::fs::File::open(if cfg!(unix) { "/dev/urandom" } else { "NUL" }) {
        let _ = std::io::Read::read_exact(&mut f, &mut id_bytes);
    }
    let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let token = launcher_capability(&key, &hex(&id_bytes), now_ms + 60_000);
    let stream = match open(&runtime.endpoint) {
        Ok(s) => s,
        Err(_) => fallback("down"),
    };
    let connected = started.elapsed();
    let langs_json = format!("[{}]", languages.iter().map(|l| quote(l)).collect::<Vec<_>>().join(","));
    let args_json = format!("[{}]", rest[1..].iter().map(|a| quote(a)).collect::<Vec<_>>().join(","));
    let cwd = std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
    let deadline = flags.iter().position(|a| a == "--deadline").and_then(|at| flags.get(at + 1)).map(|v| format!(",\"deadlineMs\":{v}")).unwrap_or_default();
    let hello = format!("{{\"t\":\"hello\",\"v\":1,\"client\":{{\"name\":\"semio-repo-forward\",\"pid\":{},\"lane\":{}}},\"languages\":{},\"cap\":{}}}\n", std::process::id(), quote(&flag("--lane", "default")), langs_json, quote(&token));
    let request = format!("{{\"t\":\"request\",\"id\":1,\"command\":{},\"args\":{},\"cwd\":{},\"env\":{{}}{}}}\n", quote(&rest[0]), args_json, quote(&cwd), deadline);
    let mut writer = &stream;
    if writer.write_all((hello + &request).as_bytes()).is_err() {
        fallback("write");
    }
    let reader = BufReader::new(&stream);
    let (stdout, stderr) = (std::io::stdout(), std::io::stderr());
    let mut accepted = false;
    let mut wrote = false;
    for line in reader.lines() {
        let Ok(line) = line else { break };
        let Some(frame) = parse_json(&line) else { fallback("protocol") };
        match frame.get("t").and_then(Json::str) {
            Some("accepted") => accepted = true,
            Some("output") => {
                wrote = true;
                let text = frame.get("text").and_then(Json::str).unwrap_or("");
                if frame.get("stream").and_then(Json::str) == Some("stderr") {
                    let _ = stderr.lock().write_all(text.as_bytes());
                } else {
                    let _ = stdout.lock().write_all(text.as_bytes());
                }
            }
            Some("result") => {
                let exit = frame.get("exit").and_then(Json::num).unwrap_or(1.0) as i32;
                if timing {
                    eprintln!("{{\"served\":{},\"client\":{{\"connectMs\":{:.3},\"resultMs\":{:.3}}}}}", quote(frame.get("served").and_then(Json::str).unwrap_or("")), connected.as_secs_f64() * 1000.0, started.elapsed().as_secs_f64() * 1000.0);
                }
                let _ = stdout.lock().flush();
                std::process::exit(exit);
            }
            Some("refused") => {
                let retry = frame.get("retry").and_then(Json::str).unwrap_or("never");
                if retry == "fallback" {
                    fallback("refused");
                }
                let message = frame.get("message");
                let text = languages.iter().find_map(|l| message.and_then(|m| m.get(l)).and_then(Json::str)).unwrap_or("refused");
                eprintln!("{text}");
                std::process::exit(if retry == "later" { 75 } else { 1 });
            }
            _ => {}
        }
    }
    if accepted || wrote {
        eprintln!("[repo] the resident service ended the connection before the command finished");
        std::process::exit(70);
    }
    fallback("closed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_fips_vectors() {
        assert_eq!(hex(&sha256(b"abc")), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(hex(&sha256(b"")), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(hex(&sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")), "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
    }

    #[test]
    fn hmac_rfc4231_vectors() {
        assert_eq!(hex(&hmac_sha256(&[0x0b; 20], b"Hi There")), "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7");
        assert_eq!(hex(&hmac_sha256(b"Jefe", b"what do ya want for nothing?")), "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843");
        assert_eq!(hex(&hmac_sha256(&[0xaa; 131], b"Test Using Larger Than Block-Size Key - Hash Key First")), "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54");
    }

    #[test]
    fn json_roundtrip() {
        let v = parse_json(r#"{"t":"output","text":"a\nä😀","n":-1.5e2,"x":[true,null]}"#).unwrap();
        assert_eq!(v.get("text").and_then(Json::str), Some("a\nä😀"));
        assert_eq!(v.get("n").and_then(Json::num), Some(-150.0));
        assert!(parse_json("{\"a\":}").is_none());
    }

    #[test]
    fn base64url_vectors() {
        assert_eq!(base64url(b""), "");
        assert_eq!(base64url(b"f"), "Zg");
        assert_eq!(base64url(b"fo"), "Zm8");
        assert_eq!(base64url(b"foo"), "Zm9v");
        assert_eq!(base64url(&[0xfb, 0xff]), "-_8");
    }
}
