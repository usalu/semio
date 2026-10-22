{
  "version": 1,
  "id": "cargo",
  "label": "Cargo",
  "ecosystem": "rust",
  "toolchain": "cargo",
  "executable": "cargo",
  "runtime": {
    "hostVersionKey": null,
    "probe": {
      "arguments": [
        "--version"
      ],
      "pattern": "^cargo (\\S+)"
    }
  },
  "manifests": [
    "Cargo.toml"
  ],
  "manifestIdentity": {
    "excludedTopLevelFields": [],
    "bootstrapFields": [
      "dependencies",
      "dev-dependencies",
      "build-dependencies",
      "workspace"
    ]
  },
  "locks": [
    {
      "name": "Cargo.lock",
      "role": "primary",
      "format": "cargo-lock"
    }
  ],
  "configurations": {
    "scope": [
      ".cargo/config.toml",
      "rust-toolchain.toml"
    ],
    "home": [
      ".cargo/config.toml"
    ],
    "config": []
  },
  "environment": {
    "set": {
      "HOME": "role:home",
      "USERPROFILE": "role:home",
      "XDG_CONFIG_HOME": "role:config",
      "XDG_CACHE_HOME": "role:cache",
      "TMPDIR": "role:temporary",
      "TMP": "role:temporary",
      "TEMP": "role:temporary",
      "CARGO_HOME": "role:cache",
      "CARGO_TARGET_DIR": "role:temporary",
      "CARGO_NET_OFFLINE": "literal:{offline}"
    }
  },
  "recipe": {
    "version": 1,
    "kind": "rust-install-projection",
    "backends": {},
    "offline": "false",
    "cache": "cargo-home",
    "global": "cargo-global",
    "bin": "cargo-bin",
    "clearEnvironmentPrefixes": [
      "CARGO_"
    ],
    "clearEnvironmentKeys": [
      "HOME",
      "USERPROFILE",
      "XDG_CONFIG_HOME",
      "XDG_CACHE_HOME",
      "TMPDIR",
      "TMP",
      "TEMP"
    ],
    "temporary": "tmp",
    "home": "home",
    "config": "config",
    "directoryRoles": [
      "cargo-home",
      "cargo-global",
      "cargo-bin",
      "tmp",
      "home",
      "config"
    ],
    "maximumEnvironmentEntries": 512,
    "maximumEnvironmentBytes": 65536,
    "maximumEnvironmentValueUnits": 32768,
    "borrowedProviders": "canonical-owner-declared-version-and-manifest"
  },
  "backendKinds": {},
  "operations": {
    "lock": {
      "arguments": [
        "update",
        "--workspace",
        "--manifest-path",
        "{manifest}"
      ]
    },
    "sync": {
      "arguments": [
        "fetch",
        "--locked",
        "--manifest-path",
        "{manifest}"
      ]
    },
    "verify": {
      "arguments": [
        "metadata",
        "--locked",
        "--format-version",
        "1",
        "--manifest-path",
        "{manifest}"
      ]
    }
  },
  "installed": {
    "containers": [
      "registry/src/*"
    ],
    "entry": "{name}-{version}",
    "match": "exact",
    "nameTransform": "identity",
    "nameEscape": {},
    "fold": "identity",
    "suffix": null,
    "sourcePrefix": "registry+"
  },
  "resolution": {
    "profiles": [],
    "strictPrefix": null,
    "oracle": "none",
    "conditions": {}
  },
  "producerInputs": {
    "scopeFiles": [
      ".cargo/config.toml",
      "rust-toolchain.toml"
    ],
    "ambientHomeFiles": [
      ".cargo/config.toml"
    ],
    "ambientConfigFiles": [],
    "ambientConfigFallback": ".config",
    "environmentPrefixes": [
      "CARGO_",
      "RUSTUP_"
    ],
    "environmentIgnored": []
  },
  "ownedModules": [
    "🗂️workspaces/🦀️cargo"
  ],
  "stagedConfiguration": "",
  "generationDirectory": "r",
  "graphComponent": null
}
