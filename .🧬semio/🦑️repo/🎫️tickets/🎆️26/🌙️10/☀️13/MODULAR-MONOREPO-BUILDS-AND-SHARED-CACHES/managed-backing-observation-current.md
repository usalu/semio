# Current Managed Backing Observation

Read-only, changing macOS host. Each stat observes a known authored image; no image opened, detached, adopted or deleted. Free-space values are observations, not capacity reservations. The partition declarations and actual known backing files are:

```json
[
  {
    "partition": "rust",
    "maximumBytes": 17179869184,
    "backings": [
      {
        "name": "fixed.dmg",
        "logicalBytes": 17178820608,
        "allocatedBytes": 16271130624
      }
    ],
    "freeBytes": 1515769856
  },
  {
    "partition": "tools",
    "maximumBytes": 8589934592,
    "backings": [
      {
        "name": "fixed.dmg",
        "logicalBytes": 8588886016,
        "allocatedBytes": 8588886016
      }
    ],
    "freeBytes": 1260744704
  },
  {
    "partition": "graph",
    "maximumBytes": 4294967296,
    "backings": [
      {
        "name": "fixed.dmg",
        "logicalBytes": 4293918720,
        "allocatedBytes": 4293918720
      }
    ],
    "freeBytes": 2034835456
  },
  {
    "partition": "dependencies",
    "maximumBytes": 8589934592,
    "backings": [
      {
        "name": "fixed.dmg",
        "logicalBytes": 8588886016,
        "allocatedBytes": 8588886016
      }
    ],
    "freeBytes": 5762580480
  },
  {
    "partition": "products",
    "maximumBytes": 25769803776,
    "backings": [
      {
        "name": "fixed.dmg",
        "logicalBytes": 25768755200,
        "allocatedBytes": 25768755200
      }
    ],
    "freeBytes": 24854970368
  },
  {
    "partition": "diagnostics",
    "maximumBytes": 2147483648,
    "backings": [
      {
        "name": "fixed.dmg",
        "logicalBytes": 2146435072,
        "allocatedBytes": 2146435072
      }
    ],
    "freeBytes": 1908330496
  },
  {
    "partition": "leases",
    "maximumBytes": 67108864,
    "backings": [
      {
        "name": "fixed.dmg",
        "logicalBytes": 66060288,
        "allocatedBytes": 66060288
      }
    ],
    "freeBytes": 5099520
  }
]
```

The authored generated envelope is64GiB plus its separately counted1MiB bootstrap control. These observations do not establish a whole-checkout maximum: source/assets/history, old unmanaged directories and remaining output writers are separate frontiers. Current enforcing platform qualification is macOS; native Linux and Windows runtime enforcement is not qualified here. Permanent resource-identity cardinality inside the finite lease image remains open pending actual authenticated namespace enrollment.
