---
name: blnbo
kind: validator
summary: Berlin Building Code (BauO Bln) validator for coda ACC.
---

# Summary

Berlin Building Code (BauO Bln) validator. Go binary reads translation JSON from stdin, validates staircase placement and building height rules, outputs report JSON to stdout.

# Specs

- Validator reads translation JSON from stdin and writes report JSON to stdout.
- Rules: staircase-located (§35 BauO Bln), building-height-limit (§2 BauO Bln high-rise threshold).
- Staircase rule checks three exemption clauses: external staircase, building class 1/2, two-storey small usage unit.
- Building height limit is 21m (Hochhausgrenze).
- Translation schema defined in blnbo.json.
