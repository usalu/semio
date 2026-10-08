# Canonical Workspace Lock Publication

First attempt published the targeted Canvas native/React Bun rows but refused before Cargo publication because Hub is an independent workspace absent from root Cargo.lock. No Cargo changes occurred in that failed attempt. The successor updated General/OS dependency lists only, checked Bun TOML against Iarna, and rebound stale Canvas consumer dependency fields from actual package declarations. Hub now directly declares its General Artifact Reference path; its separate owning lock/build is pending. Every other initial Bun workspace/package row was preserved in the first guarded write. No new external versions or migration files. Frozen resolution/native execution remain required.

{
  "cargo": [
    [
      "semio-framework",
      "semio-framework-os-kernel",
      "semio-framework-pack"
    ],
    [
      "semio-framework-os-kernel",
      null,
      "semio-framework"
    ]
  ],
  "bunConsumers": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
  ]
}

## Actual Resolution Epoch1

[
  {
    "command": "resolve",
    "code": 1,
    "childCode": 0,
    "reason": "exit",
    "exactSelected": false,
    "exactProducer": true,
    "lockExact": false,
    "advances": [
      "/Users/ueli/Documents/semio/bun.lock"
    ]
  },
  {
    "command": "frozen",
    "code": 1,
    "childCode": 0,
    "reason": "exit",
    "exactSelected": false,
    "exactProducer": true,
    "lockExact": false,
    "advances": [
      "/Users/ueli/Documents/semio/bun.lock"
    ]
  }
]

## Stable Successor Epoch2

[
  {
    "phase": "resolve-2",
    "code": 0,
    "childCode": 0,
    "reason": "exit",
    "selected": 17,
    "exactSelected": true,
    "exactProducer": true,
    "lockExact": true
  },
  {
    "phase": "frozen-2",
    "code": 0,
    "childCode": 0,
    "reason": "exit",
    "selected": 17,
    "exactSelected": true,
    "exactProducer": true,
    "lockExact": true
  }
]

Cargo actual metadata resolved current workspace and its graph; Bun actual frozen resolution returned0 without selected source advancement in this stable successor. Capture guard is scoped to Cargo lock for native owning acceptance; Bun checks do not depend on unrelated Cargo.lock.
