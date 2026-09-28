# Native Compilation Lock Checkpoint

The native test queue stopped showing compiler activity while the ZIP native-8 Cargo process retained the native artifact/directory handles and many unit lock handles. One-second macOS samples of the owned WAV and ZIP Cargo processes are retained under generated output. WAV waited at the directory lock; ZIP waited inside its job queue with a worker at `prebuild_lock_exclusive`/`flock`. ZIP had no compiler child at inspection. These observations identify lock waiting; they do not establish a compiler bug or deadlock mechanism.

Root verified PID55538 was exactly the owned ZIP native-8 no-run Cargo process and had no compiler child, then sent SIGTERM to that process only so its held locks could be released. No other chat process or lock file was changed. ZIP-8 is interrupted without a test result; its retry is held until the XML schema checkpoint is coherent. Other queued native work remains active.

Full component16 ended after30m39s with three in-flight DOCX canonical-cut compile errors: mutation helper lifetime, obsolete sync_main_part call in Transitional builder, and borrowed projection escaping a local document. The DOCX worker owns those consumers; no new full-catalog build is queued until its interface is coherent.

## XML Deliverable-Directory Isolation Probe

After XML6 spent over 45 minutes without a compiler child, root inspected only the owned Cargo processes. XML6 held handles to the shared debug build/uplift locks, while DOCX7 held a large set of unit-lock handles. This does not by itself prove a deadlock; other independent native/WASM jobs continued compiling. The repository `.cargo/config.toml` explicitly separates `build-dir` intermediates from `target-dir` deliverables and documents a private `CARGO_TARGET_DIR` as a supported way to divert only small outputs.

Root verified XML6 Cargo PID29305 had the exact expected package command and no child process, then stopped only that owned process. XML7 now runs the same registered Bun/Nx artifact test with `CARGO_TARGET_DIR=<ticket>/🗑️generated/native-xml-target` and the unchanged shared build cache. No peer process or lock file was touched. The retry has no result yet; if it makes progress, the same isolated-deliverable pattern can be used for subsequent native captures.

## Per-Unit Wait and Core Private-Uplift Retry

The read-only macOS sample of XML native 7 PID 55738 captured the Cargo worker in `prebuild_lock_exclusive` / `LockManager::lock` / `flock`; the main thread waits in the Cargo job queue. This is a shared compilation-unit lock wait, not proof of a deadlock. XML native 7 has acquired further dependency-unit handles since its early checkpoint, so it must not be reported as having made no compilation progress solely because a momentary process inventory has no rustc child. The generated sample is `🗑️generated/xml-native-7-cargo-sample.txt`.

Core native 8 PID 33698 remained at the shared uplift artifact-directory lock with no child compiler and no assertions. Root verified its exact retained_clone Cargo command and empty child set, stopped only that owned process, and launched native 9 using the same registered Nx target with a ticket-private `CARGO_TARGET_DIR` and shared intermediate build cache. Output is `🗑️generated/retained-clone-kernel-native-9.log`. No peer process, lock, or cache was changed. Native 9 has no runtime result yet.
