# Schema Dependency Regression

Registered Nx RED failed with the newly introduced dependency API absent (0 passed, 1 failed). The implementation excludes cfg(test) items and scans actual lexical references rather than comments or source strings. GREEN verification is pending.
