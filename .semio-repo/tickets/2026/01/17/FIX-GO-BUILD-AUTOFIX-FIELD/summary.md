# Summary

Removed incomplete "autofix" GraphQL field from `violationType` in `go/repo/main.go`. The field referenced an undefined `autofixType` and non-existent `Violation.Autofix` struct field. The new autofix mechanism applies fixes directly via functions rather than serializing autofix objects. Build now succeeds.
