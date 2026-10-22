{
  "version": 1,
  "id": "uv",
  "label": "uv",
  "ecosystem": "python",
  "toolchain": "python",
  "executable": "uv",
  "runtime": {
    "hostVersionKey": null,
    "probe": {
      "arguments": [
        "--version"
      ],
      "pattern": "^uv ([0-9]+\\.[0-9]+\\.[0-9]+)"
    }
  },
  "manifests": [
    "pyproject.toml"
  ],
  "manifestIdentity": {
    "excludedTopLevelFields": [],
    "bootstrapFields": [
      "project",
      "dependency-groups",
      "tool"
    ]
  },
  "locks": [
    {
      "name": "uv.lock",
      "role": "primary",
      "format": "uv-lock"
    }
  ],
  "configurations": {
    "scope": [
      "uv.toml",
      ".python-version"
    ],
    "home": [],
    "config": [
      "uv/uv.toml"
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
      "UV_CACHE_DIR": "role:cache",
      "UV_PYTHON_INSTALL_DIR": "role:global",
      "UV_PYTHON_PREFERENCE": "literal:only-managed",
      "UV_PYTHON_DOWNLOADS": "literal:manual",
      "UV_NO_CONFIG": "literal:1",
      "UV_NO_PROGRESS": "literal:1"
    }
  },
  "recipe": {
    "version": 1,
    "kind": "python-install-projection",
    "backends": {},
    "cache": "uv-cache",
    "global": "uv-python",
    "bin": "uv-bin",
    "clearEnvironmentPrefixes": [
      "UV_"
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
      "uv-cache",
      "uv-python",
      "uv-bin",
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
        "lock",
        {
          "optional": [
            "--cache-dir",
            "{cache}"
          ]
        }
      ]
    },
    "sync": {
      "arguments": [
        "sync",
        "--locked",
        {
          "optional": [
            "--project",
            "{project}"
          ]
        },
        {
          "each": "groups",
          "arguments": [
            "--group",
            "{item}"
          ]
        },
        {
          "optional": [
            "--cache-dir",
            "{cache}"
          ]
        }
      ],
      "scopes": {
        "all": [
          "--all-packages",
          "--all-groups"
        ]
      }
    },
    "provision": {
      "arguments": [
        "python",
        "install",
        "{series}"
      ]
    },
    "verify": {
      "arguments": [
        "lock",
        "--check",
        {
          "optional": [
            "--cache-dir",
            "{cache}"
          ]
        }
      ]
    }
  },
  "installed": {
    "containers": [
      ".venv/lib/*/site-packages",
      ".venv/Lib/site-packages"
    ],
    "entry": "{name}-{version}.dist-info",
    "match": "listed",
    "nameTransform": "pep-escape",
    "nameEscape": {},
    "fold": "pep-escape",
    "suffix": null,
    "sourcePrefix": null
  },
  "resolution": {
    "profiles": [],
    "strictPrefix": null,
    "oracle": "none",
    "conditions": {}
  },
  "producerInputs": {
    "scopeFiles": [
      "uv.toml",
      ".python-version"
    ],
    "ambientHomeFiles": [],
    "ambientConfigFiles": [
      "uv/uv.toml"
    ],
    "ambientConfigFallback": ".config",
    "environmentPrefixes": [
      "UV_"
    ],
    "environmentIgnored": []
  },
  "ownedModules": [],
  "stagedConfiguration": "",
  "generationDirectory": "p",
  "graphComponent": null
}
