# WFC Resumed Portable Source Validation

Executed Bun source admission on 2026-10-08. Third-party AJV validates all five portable fixture corpora. Bun and third-party Iarna TOML parse all ten current owner and package manifests identically. All original ten coordinates and ten aggregate names are conserved across child fixtures; all five provider declarations are exact, and each native binary requires its owned feature. This is static source admission, not actual native compilation or runtime output acceptance.

```json
{
  "originalCoordinates": 10,
  "originalAggregates": 10,
  "checks": [
    {
      "artifact": "s.wfc.wfc3d",
      "coordinates": 3,
      "aggregates": 3,
      "fixtureAjvValidated": true,
      "workspaceAndPackageTomlOracleExact": true
    },
    {
      "artifact": "s.wfc.grid3d",
      "coordinates": 1,
      "aggregates": 1,
      "fixtureAjvValidated": true,
      "workspaceAndPackageTomlOracleExact": true
    },
    {
      "artifact": "s.wfc.bitmap",
      "coordinates": 2,
      "aggregates": 2,
      "fixtureAjvValidated": true,
      "workspaceAndPackageTomlOracleExact": true
    },
    {
      "artifact": "s.wfc.wfc2d",
      "coordinates": 3,
      "aggregates": 3,
      "fixtureAjvValidated": true,
      "workspaceAndPackageTomlOracleExact": true
    },
    {
      "artifact": "s.wfc.grid2d",
      "coordinates": 1,
      "aggregates": 1,
      "fixtureAjvValidated": true,
      "workspaceAndPackageTomlOracleExact": true
    }
  ]
}
```

Current five lock registry/source package blocks also compare byte for byte equal to their retained original preimages. Only own package dependency/feature publications differ.

```json
[
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.lock",
    "foreignEntriesByteIdentical": true,
    "foreignCount": 0,
    "beforeSha256": "c9967a20589a0172848daf3daea87f31e43a6482732506b37dc13f596d1c914d",
    "currentSha256": "ad7111df3308598d349d7a5738a5908f96ef465f928264214b7d91bcc53c1485"
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.lock",
    "foreignEntriesByteIdentical": true,
    "foreignCount": 0,
    "beforeSha256": "61b181599a2ada95f5b920b9946c53474c41734b3353e143c10c4ae5cfae9b85",
    "currentSha256": "2b608de41386f02d6a0ffd156e18cf6557fe6d6bf96fa7db39c3e524dc7ab288"
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.lock",
    "foreignEntriesByteIdentical": true,
    "foreignCount": 0,
    "beforeSha256": "ad40a638928932f9d5de531f8cf0a05f2c1259c2654aacd48e20fd0933f5dbf8",
    "currentSha256": "de9846362140d24582bc31135e6d1b42c5019c1d76a2bf85b0568a1bb29e229d"
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.lock",
    "foreignEntriesByteIdentical": true,
    "foreignCount": 0,
    "beforeSha256": "24d7011fe9942dfd807d51a5152ad402569dfea2642b35ef10095f96c2222312",
    "currentSha256": "716ab1de1ed61c19df133356255cd88214ecff6ea389e352343eeb7a8d9db938"
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.lock",
    "foreignEntriesByteIdentical": true,
    "foreignCount": 0,
    "beforeSha256": "b88216c70f344294dc69d900dd03b823c636d7723931fdbbb69e7ca8562968d9",
    "currentSha256": "e0eaa01b5d84d19b536287b92483be9ca25e1c29b6989babc9862161ceebf380"
  }
]
```

All ten complete retained aggregate/snapshot type pairs equal the direct child `DESCRIPTORS` pairings, including document and editor aggregates. Executed source comparison passed; native execution remains pending.
