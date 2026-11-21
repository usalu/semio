//! 🧾 Transport the exact PDF law result through either the native filesystem or WASI stdout.

use std::path::Path;

pub fn save_output(work: &Path, bytes: &[u8]) {
    #[cfg(not(target_arch = "wasm32"))]
    std::fs::write(work.join("out.pdf"), bytes).unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        use std::io::Write;
        let _ = work;
        let mut output = std::io::stdout().lock();
        writeln!(output, "SEMIO_PDF_SAVE_BYTES {}", bytes.len()).unwrap();
        let alphabet = b"0123456789abcdef";
        let mut encoded = [0; 8192];
        for chunk in bytes.chunks(encoded.len() / 2) {
            for (byte, pair) in chunk.iter().zip(encoded.chunks_exact_mut(2)) {
                pair[0] = alphabet[usize::from(byte >> 4)];
                pair[1] = alphabet[usize::from(byte & 15)];
            }
            output.write_all(&encoded[..chunk.len() * 2]).unwrap();
        }
        output.flush().unwrap();
    }
}
