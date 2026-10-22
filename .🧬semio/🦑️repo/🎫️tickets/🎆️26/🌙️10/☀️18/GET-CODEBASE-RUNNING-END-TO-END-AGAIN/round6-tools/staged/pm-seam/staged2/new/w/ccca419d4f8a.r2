{
  "version": 1,
  "id": "bun",
  "label": "Bun",
  "ecosystem": "javascript",
  "toolchain": "javascript",
  "executable": "bun",
  "runtime": {
    "hostVersionKey": "bun",
    "probe": {
      "arguments": [
        "--version"
      ],
      "pattern": "^([0-9]+\\.[0-9]+\\.[0-9]+)"
    }
  },
  "manifests": [
    "package.json"
  ],
  "manifestIdentity": {
    "excludedTopLevelFields": [
      "scripts",
      "semio"
    ],
    "bootstrapFields": [
      "dependencies",
      "devDependencies",
      "optionalDependencies",
      "patchedDependencies"
    ]
  },
  "locks": [
    {
      "name": "bun.lock",
      "role": "primary",
      "format": "bun-lock"
    },
    {
      "name": "bun.lockb",
      "role": "secondary",
      "format": "bun-lockb"
    }
  ],
  "configurations": {
    "scope": [
      "bunfig.toml"
    ],
    "home": [
      ".bunfig.toml",
      ".npmrc"
    ],
    "config": [
      ".bunfig.toml"
    ]
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
      "NODE_ENV": "literal:{nodeEnvironment}",
      "BUN_INSTALL_CACHE_DIR": "role:cache",
      "BUN_INSTALL_GLOBAL_DIR": "role:global",
      "BUN_INSTALL_BIN": "role:bin"
    }
  },
  "recipe": {
    "version": 6,
    "kind": "javascript-install-projection",
    "backends": {
      "darwin": "clonefile",
      "linux": "copyfile",
      "win32": "copyfile"
    },
    "lifecycle": "ignore-scripts",
    "linker": "isolated",
    "cache": "bun-cache",
    "global": "bun-global",
    "bin": "bun-bin",
    "temporary": "tmp",
    "home": "bun-home",
    "config": "bun-config",
    "clearEnvironmentPrefixes": [
      "BUN_",
      "NPM_CONFIG_"
    ],
    "clearEnvironmentKeys": [
      "HOME",
      "USERPROFILE",
      "XDG_CONFIG_HOME",
      "XDG_CACHE_HOME",
      "TMPDIR",
      "TMP",
      "TEMP",
      "NODE_ENV",
      "NODE_OPTIONS"
    ],
    "directoryRoles": [
      "bun-cache",
      "bun-global",
      "bun-bin",
      "tmp",
      "bun-home",
      "bun-config"
    ],
    "nodeEnvironment": "development",
    "maximumEnvironmentEntries": 512,
    "maximumEnvironmentBytes": 65536,
    "maximumEnvironmentValueUnits": 32768,
    "borrowedProviders": "canonical-owner-declared-version-and-manifest"
  },
  "backendKinds": {
    "clonefile": "clone",
    "copyfile": "copy",
    "hardlink": "alias",
    "symlink": "alias"
  },
  "operations": {
    "lock": {
      "arguments": [
        "install",
        "--lockfile-only",
        "--{lifecycle}",
        "--linker={linker}",
        "--backend={backend}",
        "--cache-dir",
        "{cache}"
      ]
    },
    "sync": {
      "arguments": [
        "install",
        "--frozen-lockfile",
        "--{lifecycle}",
        "--linker={linker}",
        "--backend={backend}",
        "--cache-dir",
        "{cache}"
      ]
    },
    "verify": {
      "arguments": [
        "install",
        "--frozen-lockfile",
        "--dry-run",
        "--{lifecycle}",
        "--linker={linker}",
        "--backend={backend}",
        "--cache-dir",
        "{cache}"
      ]
    },
    "install-pinned": {
      "arguments": [
        "install",
        "--frozen-lockfile",
        "--{lifecycle}",
        "--cache-dir",
        "{cache}",
        "--backend",
        "{backend}"
      ],
      "values": {
        "backend": "copyfile"
      }
    }
  },
  "installed": {
    "containers": [
      "node_modules/.bun"
    ],
    "entry": "{name}@{version}",
    "match": "listed",
    "nameTransform": "identity",
    "nameEscape": {
      "/": "+"
    },
    "fold": "identity",
    "suffix": "^(\\+[0-9a-f]{16})?$",
    "sourcePrefix": null
  },
  "resolution": {
    "profiles": [
      "bun-import",
      "bun-require"
    ],
    "strictPrefix": "bun-",
    "oracle": "bun",
    "conditions": {
      "bun-import": [
        "bun",
        "node-addons",
        "node",
        "import",
        "default"
      ],
      "bun-require": [
        "bun",
        "node-addons",
        "node",
        "require",
        "default"
      ]
    }
  },
  "producerInputs": {
    "scopeFiles": [
      ".npmrc"
    ],
    "ambientHomeFiles": [
      ".npmrc",
      ".bunfig.toml"
    ],
    "ambientConfigFiles": [
      ".bunfig.toml"
    ],
    "ambientConfigFallback": ".config",
    "environmentPrefixes": [
      "BUN_",
      "NPM_CONFIG_"
    ],
    "environmentIgnored": [
      "NPM_CONFIG_USER_AGENT",
      "BUN_INSPECT",
      "BUN_INSPECT_CONNECT_TO",
      "BUN_INSPECT_NOTIFY"
    ]
  },
  "ownedModules": [
    "🏗️build/🕸️graph/📥️inference/🧶️bun-lock/🟨️.mjs",
    "🗂️workspaces/🟦️bun"
  ],
  "stagedConfiguration": "[install]\nlinker = \"isolated\"\nglobalStore = false\npublicHoistPattern = []\nhoistPattern = []\n",
  "generationDirectory": "j",
  "graphComponent": "bun-lock"
}
