{
  "version": 1,
  "id": "go",
  "label": "Go",
  "ecosystem": "go",
  "toolchain": "go",
  "executable": "go",
  "runtime": {
    "hostVersionKey": null,
    "probe": {
      "arguments": [
        "version"
      ],
      "pattern": "^go version go(\\S+)"
    }
  },
  "manifests": [
    "go.mod",
    "go.work"
  ],
  "manifestIdentity": {
    "excludedTopLevelFields": [],
    "bootstrapFields": []
  },
  "locks": [
    {
      "name": "go.sum",
      "role": "primary",
      "format": "go-sum"
    },
    {
      "name": "go.work.sum",
      "role": "secondary",
      "format": "go-sum"
    }
  ],
  "configurations": {
    "scope": [
      "go.work"
    ],
    "home": [],
    "config": [
      "go/env"
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
      "GOMODCACHE": "role:cache",
      "GOPATH": "role:global",
      "GOFLAGS": "literal:{flags}",
      "GOTOOLCHAIN": "literal:local",
      "GOPROXY": "literal:{proxy}"
    }
  },
  "recipe": {
    "version": 1,
    "kind": "go-install-projection",
    "backends": {},
    "flags": "-mod=readonly",
    "proxy": "https://proxy.golang.org,direct",
    "cache": "go-mod-cache",
    "global": "go-path",
    "bin": "go-bin",
    "clearEnvironmentPrefixes": [
      "GO"
    ],
    "clearEnvironmentKeys": [
      "HOME",
      "USERPROFILE",
      "XDG_CONFIG_HOME",
      "XDG_CACHE_HOME",
      "TMPDIR",
      "TMP",
      "TEMP",
      "GOFLAGS"
    ],
    "temporary": "tmp",
    "home": "home",
    "config": "config",
    "directoryRoles": [
      "go-mod-cache",
      "go-path",
      "go-bin",
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
        "mod",
        "tidy"
      ]
    },
    "sync": {
      "arguments": [
        "mod",
        "download"
      ]
    },
    "verify": {
      "arguments": [
        "mod",
        "verify"
      ]
    }
  },
  "installed": {
    "containers": [
      "."
    ],
    "entry": "{name}@{version}",
    "match": "exact",
    "nameTransform": "go-escape",
    "nameEscape": {},
    "fold": "identity",
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
      "go.work"
    ],
    "ambientHomeFiles": [],
    "ambientConfigFiles": [
      "go/env"
    ],
    "ambientConfigFallback": ".config",
    "environmentPrefixes": [
      "GO",
      "CGO_"
    ],
    "environmentIgnored": []
  },
  "ownedModules": [],
  "stagedConfiguration": "",
  "generationDirectory": "g",
  "graphComponent": null
}
