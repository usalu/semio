{
  "version": 1,
  "id": "dotnet",
  "label": ".NET",
  "ecosystem": "dotnet",
  "toolchain": "dotnet",
  "executable": "dotnet",
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
    "Directory.Packages.props",
    "global.json"
  ],
  "manifestIdentity": {
    "excludedTopLevelFields": [],
    "bootstrapFields": []
  },
  "locks": [
    {
      "name": "packages.lock.json",
      "role": "primary",
      "format": "nuget-lock"
    }
  ],
  "configurations": {
    "scope": [
      "NuGet.Config",
      "nuget.config",
      "global.json"
    ],
    "home": [
      ".nuget/NuGet/NuGet.Config"
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
      "NUGET_PACKAGES": "role:cache",
      "DOTNET_CLI_HOME": "role:home",
      "DOTNET_CLI_TELEMETRY_OPTOUT": "literal:1",
      "DOTNET_NOLOGO": "literal:1"
    }
  },
  "recipe": {
    "version": 1,
    "kind": "dotnet-install-projection",
    "backends": {},
    "cache": "nuget-packages",
    "global": "dotnet-global",
    "bin": "dotnet-bin",
    "clearEnvironmentPrefixes": [
      "DOTNET_",
      "NUGET_"
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
      "nuget-packages",
      "dotnet-global",
      "dotnet-bin",
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
        "restore",
        "{manifest}",
        "--use-lock-file",
        "--packages",
        "{cache}"
      ]
    },
    "sync": {
      "arguments": [
        "restore",
        "{manifest}",
        "--locked-mode",
        "--packages",
        "{cache}"
      ]
    }
  },
  "installed": {
    "containers": [
      "."
    ],
    "entry": "{name}/{version}",
    "match": "exact",
    "nameTransform": "lowercase",
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
      "NuGet.Config",
      "nuget.config",
      "global.json"
    ],
    "ambientHomeFiles": [
      ".nuget/NuGet/NuGet.Config"
    ],
    "ambientConfigFiles": [],
    "ambientConfigFallback": ".config",
    "environmentPrefixes": [
      "DOTNET_",
      "NUGET_"
    ],
    "environmentIgnored": []
  },
  "ownedModules": [],
  "stagedConfiguration": "",
  "generationDirectory": "n",
  "graphComponent": null
}
