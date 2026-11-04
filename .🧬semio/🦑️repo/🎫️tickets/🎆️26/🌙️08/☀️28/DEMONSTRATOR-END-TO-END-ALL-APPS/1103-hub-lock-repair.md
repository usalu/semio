# Hub Lock Repair

Third-party Python toml independently validates the raw-block repair against all original package records and references.

- Fixture cases passed: 3
- Before duplicate rows: 114
- Removed identical package bodies: 114
- After duplicate identities: 0
- Original SHA256: 9aa3bedd197f6399b1082dbb5e4572754d767c07c190fca3e8226e8e6591b26d
- Candidate SHA256: 198ac483f21c02a768f253f012fd2a16197224fb248e63a7c98378734539f7cf
- All retained package bodies, dependency references, and non-package metadata preserved.
- Final block separator whitespace is excluded from package body equality; no package text is normalized.

Applied only to 🌎️hub/Cargo.lock.
