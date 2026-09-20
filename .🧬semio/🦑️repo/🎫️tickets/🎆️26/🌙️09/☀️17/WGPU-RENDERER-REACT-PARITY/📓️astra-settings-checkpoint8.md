# Checkpoint8 Settings Runtime Failure

The new paired five-step Settings journey was run against the same completed checkpoint8 wasm. React opens Settings, selects General and publishes Appearance; WGPU fails the General outcome after15seconds and an additional15second settle. This is a real recorded failure, not an absent action-ledger signal.

## settings-open

- react: error=None; operationMs=136; detail={"resolved": "exact", "id": "framework.settings", "rect": [1271, 974, 82, 22], "clicked": true}
- wgpu: error=None; operationMs=6; detail={"resolved": "exact", "id": "framework.settings", "rect": [1381.4585, 974.4, 67.696655, 22.4], "clicked": true}
- Settling: {"elapsedMs": 2053, "firstPublicationAfterOperationMs": 937, "firstSemanticChangeAfterOperationMs": 937, "generationBefore": 14, "generationAfter": 15, "publicationAdvanced": true}

## settings-general

- react: error=None; operationMs=211; detail={"resolved": "exact", "id": "framework.settings.general", "rect": [1378, 953, 77, 22], "clicked": true}
- wgpu: error='Error: General settings did not publish its Appearance control'; operationMs=15038; detail=null
- Settling: {"elapsedMs": 15209, "firstPublicationAfterOperationMs": 273, "firstSemanticChangeAfterOperationMs": null, "generationBefore": 17, "generationAfter": 34, "publicationAdvanced": true}

## settings-close

- react: error=None; operationMs=192; detail={"resolved": "exact", "id": "framework.settings", "rect": [1271, 975, 82, 22], "clicked": true}
- wgpu: error=None; operationMs=11; detail={"resolved": "exact", "id": "framework.settings", "rect": [1300, 764.8, 67.69668, 22.4], "clicked": true}
- Settling: {"elapsedMs": 1700, "firstPublicationAfterOperationMs": 306, "firstSemanticChangeAfterOperationMs": 306, "generationBefore": 35, "generationAfter": 36, "publicationAdvanced": true}

## Confirmed Host Ingress

```text
28299 worker log [DEBUG] wgpu-shell pointer button x=1415.3069 y=985.6 down=true button=0 targets=42 staged=39 gen=14 hit=Some((Toggle, Some("framework.settings"), None))
28621 worker log [DEBUG] wgpu-shell pointer button x=1415.3069 y=985.6 down=false button=0 targets=42 staged=39 gen=14 hit=Some((Toggle, Some("framework.settings"), None))
32620 worker log [DEBUG] wgpu-shell pointer button x=1402.379 y=798.4 down=true button=0 targets=50 staged=50 gen=17 hit=Some((PanelTab, Some("framework.settings.general"), None))
32653 worker log [DEBUG] wgpu-shell pointer button x=1402.379 y=798.4 down=false button=0 targets=50 staged=50 gen=17 hit=Some((PanelTab, Some("framework.settings.general"), None))
65135 worker log [DEBUG] wgpu-shell pointer button x=1333.8484 y=776 down=true button=0 targets=50 staged=50 gen=35 hit=Some((PanelTab, Some("framework.settings"), None))
65153 worker log [DEBUG] wgpu-shell pointer button x=1333.8484 y=776 down=false button=0 targets=50 staged=50 gen=35 hit=Some((PanelTab, Some("framework.settings"), None))
```

The General down/up both resolve the actual PanelTab framework.settings.general at generation17; the stale-coordinate hypothesis does not explain these events. Earlier Settings-open publication put General at y51.2 and later publication moved it to y787.2; the operation itself resolved/clicked the later current coordinates. The separate first-frame sizing movement still merits a geometry audit.

The settings-close generic locator resolves the first of duplicate framework.settings IDs (panel branch, rather than footer switch), collapsing branch children without closing the panel; this probe step needs semantic kind resolution before it can be an acceptance gate.

Native renderer11 also fails the new accessibility activation→checked-state projection law. Terra audits both paths before any handler change is assumed. The app settings leaf is blank and manual paint reports a validation fault; a producer fault is a separate possible defect requiring exact diagnosis.

Artifacts: 🗑️generated/astra-runtime/paired-checkpoint-8-settings.
