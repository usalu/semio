{
  "version": 1,
  "id": "npm",
  "label": "npm",
  "ecosystem": "javascript",
  "toolchain": "javascript",
  "executable": "npm",
  "runtime": {
    "hostVersionKey": null,
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
      "overrides"
    ]
  },
  "locks": [
    {
      "name": "package-lock.json",
      "role": "primary",
      "format": "package-lock"
    },
    {
      "name": "npm-shrinkwrap.json",
      "role": "secondary",
      "format": "package-lock"
    }
  ],
  "configurations": {
    "scope": [
      ".npmrc"
    ],
    "home": [
      ".npmrc"
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
      "NODE_ENV": "literal:{nodeEnvironment}",
      "npm_config_cache": "role:cache",
      "npm_config_prefix": "role:global"
    }
  },
  "recipe": {
    "version": 1,
    "kind": "javascript-install-projection",
    "backends": {},
    "lifecycle": "ignore-scripts",
    "strategy": "hoisted",
    "cache": "npm-cache",
    "global": "npm-global",
    "bin": "npm-bin",
    "nodeEnvironment": "development",
    "clearEnvironmentPrefixes": [
      "npm_config_",
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
    "temporary": "tmp",
    "home": "home",
    "config": "config",
    "directoryRoles": [
      "npm-cache",
      "npm-global",
      "npm-bin",
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
        "install",
        "--package-lock-only",
        "--{lifecycle}",
        "--install-strategy={strategy}",
        "--cache",
        "{cache}"
      ]
    },
    "sync": {
      "arguments": [
        "ci",
        "--{lifecycle}",
        "--install-strategy={strategy}",
        "--cache",
        "{cache}"
      ]
    },
    "verify": {
      "arguments": [
        "ci",
        "--dry-run",
        "--{lifecycle}",
        "--install-strategy={strategy}",
        "--cache",
        "{cache}"
      ]
    },
    "install-pinned": {
      "arguments": [
        "ci",
        "--{lifecycle}",
        "--cache",
        "{cache}"
      ]
    }
  },
  "installed": {
    "containers": [
      "node_modules"
    ],
    "entry": "{name}",
    "match": "exact",
    "nameTransform": "identity",
    "nameEscape": {},
    "fold": "identity",
    "suffix": null,
    "sourcePrefix": null
  },
  "resolution": {
    "profiles": [
      "node-import",
      "node-require"
    ],
    "strictPrefix": null,
    "oracle": "node",
    "conditions": {
      "node-import": [
        "node-addons",
        "node",
        "import",
        "default"
      ],
      "node-require": [
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
      ".npmrc"
    ],
    "ambientConfigFiles": [],
    "ambientConfigFallback": ".config",
    "environmentPrefixes": [
      "npm_config_",
      "NPM_CONFIG_"
    ],
    "environmentIgnored": [
      "NPM_CONFIG_USER_AGENT"
    ]
  },
  "ownedModules": [],
  "stagedConfiguration": "",
  "generationDirectory": "j",
  "graphComponent": null
}
