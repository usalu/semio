# Independent Python Assertion Inventory

Parsed 532 Python source files outside canonical cases and declared generated/opaque trees with Python AST. The following 15 files are candidates for review; runtime assertions alone do not prove that a production module is a test. This inventory is not a final test-layout verdict.

```json
[
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/review/2026-06-06_full_graph_verification/_build_element_proof_agent_02.py",
    "assertions": 1,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      319
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/review/2026-06-06_full_graph_verification/_agent11_build_ledger.py",
    "assertions": 5,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      322,
      323,
      324,
      325,
      326
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/review/2026-06-06_full_graph_verification/_ier_b2_build.py",
    "assertions": 1,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      141
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/review/2026-06-06_full_graph_verification/_ier_c5_build.py",
    "assertions": 1,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      339
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/review/2026-06-06_full_graph_verification/_ier_c4_build.py",
    "assertions": 1,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      728
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/review/2026-08_akteursnetz_faktencheck/complete_missing_info_hunt.py",
    "assertions": 2,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      428,
      429
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/review/2026-08_akteursnetz_faktencheck/apply_sharpness_upgrades.py",
    "assertions": 2,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      34,
      38
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/review/2026-08_akteursnetz_faktencheck/finalize_kanten_report.py",
    "assertions": 18,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      57,
      58,
      59,
      60,
      61,
      62,
      63,
      64,
      68,
      69,
      80,
      89,
      95,
      96,
      97
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-21_quelle_remediation/agent_s2_url_prober/logs/agent_s2_runner.py",
    "assertions": 1,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      415
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-20_radical_quality_reset/logs/migrate_helper_p1_2_3.py",
    "assertions": 17,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      315,
      316,
      317,
      318,
      319,
      320,
      321,
      322,
      323,
      332,
      335,
      268,
      271,
      275,
      282
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-20_radical_quality_reset/logs/agent5_runner.py",
    "assertions": 44,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      428,
      429,
      430,
      431,
      432,
      433,
      434,
      473,
      474,
      510,
      511,
      512,
      564,
      565,
      566
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-20_radical_quality_reset/logs/agent10_research_registry_loader.py",
    "assertions": 2,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      1050,
      1155
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-20_radical_quality_reset/logs/agent8_runner.py",
    "assertions": 4,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      463,
      466,
      470,
      475
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-20_radical_quality_reset/logs/agent6_runner.py",
    "assertions": 5,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      986,
      990,
      1004,
      1009,
      1000
    ]
  },
  {
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-20_radical_quality_reset/logs/agent7_runner.py",
    "assertions": 18,
    "testDeclarations": [],
    "frameworkImports": [],
    "assertionLines": [
      691,
      692,
      693,
      694,
      695,
      696,
      697,
      698,
      699,
      700,
      701,
      705,
      709,
      727,
      731
    ]
  }
]
```

## Classification

All 15 candidates were inspected at their enclosing function/module scope. The build-ledger, scope-loader, and report-finalization files enforce input/output invariants during their actual research transformations. The URL prober checks its response state. The six historical graph migration/registry runners enforce preconditions and postconditions of their service writes. None imports a test framework or declares a test function, and none is an isolated regression suite. These runtime assertions remain with the operations they guard; those service-mutating programs were not executed. The two actual stage parity suites discovered in fixture helpers were handled separately in the native follow-up report.
