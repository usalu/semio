use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// 🐍️ The repo-resolved Python interpreter: `SEMIO_PYTHON` (set by the repo command layer for every test target), else the nearest ancestor `.venv`; panics loudly, never falls back to a bare `python3`.
pub fn interpreter() -> PathBuf {
    if let Some(value) = std::env::var_os("SEMIO_PYTHON").filter(|value| !value.is_empty()) {
        let path = PathBuf::from(value);
        assert!(path.is_file(), "SEMIO_PYTHON={} is not a file; run `repo dependencies ensure`", path.display());
        return path;
    }
    let mut dir = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    loop {
        for relative in [".venv/Scripts/python.exe", ".venv/bin/python"] {
            let candidate = dir.join(relative);
            if candidate.is_file() {
                return candidate;
            }
        }
        assert!(dir.pop(), "no Python interpreter: SEMIO_PYTHON is unset and no ancestor .venv exists; run `repo dependencies ensure`");
    }
}

/// 🧰️ An isolated (`-I`) command on the resolved interpreter, ready for a script path or `-c` code.
pub fn command() -> Command {
    let mut command = Command::new(interpreter());
    command.arg("-I");
    command
}

/// 🚀️ Run `script` with `args`, feed `stdin` when given and return the captured output; spawn failure panics with the interpreter path.
pub fn run(script: impl AsRef<OsStr>, args: &[&OsStr], stdin: Option<&[u8]>) -> Output {
    let mut child = command()
        .arg(script)
        .args(args)
        .stdin(if stdin.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("python oracle spawn failed with {}: {error}", interpreter().display()));
    if let Some(bytes) = stdin {
        child.stdin.take().expect("piped stdin").write_all(bytes).expect("write oracle stdin");
    }
    child.wait_with_output().expect("wait for python oracle")
}

/// ✅️ Like [`run`] but panics with stderr unless the process exits successfully.
pub fn run_ok(script: impl AsRef<OsStr>, args: &[&OsStr], stdin: Option<&[u8]>) -> Output {
    let output = run(script, args, stdin);
    assert!(output.status.success(), "python oracle failed: {}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    output
}

/// 🏃️ The shared Eurocode oracle harness `🔮️oracles/🏃️harness/🐍️.py`, found from the nearest ancestor that contains it.
pub fn harness() -> PathBuf {
    let mut dir = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    loop {
        let candidate = dir.join("🔮️oracles/🏃️harness/🐍️.py");
        if candidate.is_file() {
            return candidate;
        }
        assert!(dir.pop(), "oracle harness 🔮️oracles/🏃️harness/🐍️.py not found above {}", env!("CARGO_MANIFEST_DIR"));
    }
}

/// 🔮️ Recompute every case file (`*.json`) in `dir` with the third-party libraries and fail loudly on any mismatch against the blessed expectations or between libraries; returns the number of verified cases.
pub fn verify_cases(dir: &Path) -> usize {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("oracle case directory {}: {error}", dir.display()))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "json"))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no oracle case files in {}", dir.display());
    let mut args: Vec<&OsStr> = vec![OsStr::new("--cases")];
    args.extend(files.iter().map(|file| file.as_os_str()));
    let output = run(harness(), &args, None);
    let report = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success() && !report.lines().any(|line| line.starts_with("FAIL")), "oracle mismatch:\n{report}\n{}", String::from_utf8_lossy(&output.stderr));
    report.lines().filter(|line| line.starts_with("ok ")).count()
}

/// 🧭️ [`verify_cases`] on the oracle case directory `🧪️tests/🧫️fixtures/🔮️oracle-cases` of the artifact leaf above `manifest_dir` (pass `env!("CARGO_MANIFEST_DIR")` of the calling crate); an artifact crate directory (`🏅️standards/🔖️1/🪆️subsets/✳️any/…` below an ancestor) resolves through that ancestor, so crates mounted at the artifact root see their own cases and not a shared parent directory.
pub fn verify_cases_above(manifest_dir: &str) -> usize {
    let mut dir = Path::new(manifest_dir).to_path_buf();
    loop {
        for relative in ["🧪️tests/🧫️fixtures/🔮️oracle-cases", "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"] {
            let candidate = dir.join(relative);
            if candidate.is_dir() {
                return verify_cases(&candidate);
            }
        }
        assert!(dir.pop(), "no 🧪️tests/🧫️fixtures/🔮️oracle-cases above {manifest_dir}");
    }
}
