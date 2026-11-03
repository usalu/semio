//! ✅️ Validation: every scene type reports path-addressed rule violations instead of panicking.

records! {
    /// 🚩️ One violated rule at a JSON-path-like location inside the validated value.
    pub struct Violation {
        pub path: String,
        pub rule: String,
    }
}

/// 🔍️ Structural validation; `check` accumulates every violation below `path`.
pub trait Validate {
    fn check(&self, path: &str, out: &mut Vec<Violation>);

    fn validate(&self) -> Result<(), Vec<Violation>> {
        let mut out = Vec::new();
        self.check("$", &mut out);
        if out.is_empty() { Ok(()) } else { Err(out) }
    }
}

pub(crate) const TOLERANCE: f64 = 1e-6;

pub(crate) fn at(path: &str, name: &str) -> String {
    format!("{path}.{name}")
}

pub(crate) fn nth(path: &str, name: &str, index: usize) -> String {
    format!("{path}.{name}[{index}]")
}

pub(crate) fn expect(out: &mut Vec<Violation>, path: &str, ok: bool, rule: &str) {
    if !ok {
        out.push(Violation { path: path.to_owned(), rule: rule.to_owned() });
    }
}

pub(crate) fn finite(out: &mut Vec<Violation>, path: &str, values: &[f64]) {
    expect(out, path, values.iter().all(|v| v.is_finite()), "finite");
}

pub(crate) fn positive(out: &mut Vec<Violation>, path: &str, value: f64) {
    expect(out, path, value.is_finite() && value > 0.0, "positive");
}

pub(crate) fn non_negative(out: &mut Vec<Violation>, path: &str, value: f64) {
    expect(out, path, value.is_finite() && value >= 0.0, "non-negative");
}

pub(crate) fn non_empty(out: &mut Vec<Violation>, path: &str, value: &str) {
    expect(out, path, !value.trim().is_empty(), "non-empty");
}

pub(crate) fn each<T: Validate>(items: &[T], path: &str, name: &str, out: &mut Vec<Violation>) {
    for (index, item) in items.iter().enumerate() {
        item.check(&nth(path, name, index), out);
    }
}

impl<T: Validate> Validate for Option<T> {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        if let Some(value) = self {
            value.check(path, out);
        }
    }
}

impl<T: Validate> Validate for Box<T> {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        (**self).check(path, out);
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        for (index, item) in self.iter().enumerate() {
            item.check(&format!("{path}[{index}]"), out);
        }
    }
}
