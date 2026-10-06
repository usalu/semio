# Framework Store Test Corpus Ownership

Seven Store and Presence JSON corpora encode test inputs and expected lifecycle outcomes. They now belong to canonical fixture folders at their Store or Presence semantic scope. Their schema declarations remain unchanged. All Rust includes and the Group visibility and CAD presence TypeScript readers now use those current paths.

## Preserved Moves

```json
[
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📢️member-publication.json",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/📢️member-publication.json",
    "sha256": "9e6f570d9abb7182dab6c91b42dc1066f7269a1bb22ffd6f1bddfc48ac7fa61d"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎯️group-cursor.json",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🎯️group-cursor.json",
    "sha256": "cdca67b3476daacb9e97fdf5e99189d9a5eb28c7937f48b2646cc10640375752"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📖️group-read.json",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/📖️group-read.json",
    "sha256": "26f26600a4ffe1d291e98cab4320e9e88b40b4b1f8c5acb5b9b0b173cffc198c"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🌱️runtime-seed.json",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🌱️runtime-seed.json",
    "sha256": "a07b885b5b47100f5b28db9ac9a66c77150b6942ef5c4b95fda1a1945af7843c"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧹️retirement.json",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧫️fixtures/🧹️retirement.json",
    "sha256": "db963d872f240b370f9d2fd908e53d9cc453e004d001632a279299788ffaddc8"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📌️peer-commit.json",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧫️fixtures/📌️peer-commit.json",
    "sha256": "982346b4797b63cb89c40de1ae2b6275de045aa78dcecbe9ff0111ef17936704"
  },
  {
    "old": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🛂️peer-admission.json",
    "new": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧫️fixtures/🛂️peer-admission.json",
    "sha256": "d2d9dc9ab0edebbe978f7bf232c804347962e71be02c71b61e48396312602294"
  }
]
```

## Authored Paths

```json
[
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🌱️runtime-seed.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎯️group-cursor.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📌️peer-commit.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🛂️peer-admission.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧫️fixtures/📌️peer-commit.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧫️fixtures/🛂️peer-admission.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧫️fixtures/🧹️retirement.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧹️retirement.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📖️group-read.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📢️member-publication.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🌱️runtime-seed.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🎯️group-cursor.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/📖️group-read.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/📢️member-publication.json"
]
```
