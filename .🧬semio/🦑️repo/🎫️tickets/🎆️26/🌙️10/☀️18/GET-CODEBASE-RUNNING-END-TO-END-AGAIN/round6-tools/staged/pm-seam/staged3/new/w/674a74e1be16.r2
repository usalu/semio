[
 [
  "/description",
  "Version 2 immutable Bun installation material uses canonical JSON manifests with only top-level scripts and semio omitted. Native fields other than scripts, nested semio, the install-inert manifest fields named by the adopt protocol, array ordering, lock/config/patch bytes and exact canonical physical membership remain install inputs. Every operation captures and revalidates full original Source bytes and physical path/mode facts, including omitted metadata; an active publisher witness fences complete current membership before reuse and publication. No earlier generation protocol is admitted."
 ],
 [
  "/$defs/SourceOwnerExportsProtocol/const/profiles",
  {
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
   ],
   "typescript-bundler-import": [
    "types",
    "import",
    "default"
   ]
  }
 ],
 [
  "/$defs/SourceOwnerExportsProtocol/const/arrays",
  "ordered-bun-first-null-or-invalid-target-refusal-and-typescript-null-or-invalid-target-fallback"
 ],
 [
  "/$defs/SourceOwnerExportsCase/properties/profile/enum",
  [
   "bun-import",
   "bun-require",
   "typescript-bundler-import"
  ]
 ],
 [
  "/$defs/SourceOwnerExportsCase/properties/oracle/enum",
  [
   "bun",
   "typescript",
   "node",
   "node-missing",
   "none"
  ]
 ],
 [
  "/$defs/ProducerConfigurationProtocol/const/text",
  "[install]\nlinker = \"isolated\"\nglobalStore = false\npublicHoistPattern = []\nhoistPattern = []\n"
 ],
 [
  "/$defs/ProducerRecipeProtocol/const",
  {
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
  }
 ],
 [
  "/$defs/ProducerDirectoryRoleCase/properties/name/enum/[{{recipe.cache}}]",
  "bun-cache"
 ],
 [
  "/$defs/ProducerDirectoryRoleCase/properties/name/enum/[{{recipe.global}}]",
  "bun-global"
 ],
 [
  "/$defs/ProducerDirectoryRoleCase/properties/name/enum/[{{recipe.bin}}]",
  "bun-bin"
 ],
 [
  "/$defs/ProducerDirectoryRoleCase/properties/name/enum/[{{recipe.home}}]",
  "bun-home"
 ],
 [
  "/$defs/ProducerDirectoryRoleCase/properties/name/enum/[{{recipe.config}}]",
  "bun-config"
 ],
 [
  "/$defs/DelayedCustody/properties/result/properties/refusal/const",
  "Original Bun installation callback released"
 ],
 [
  "/$defs/SelectedPlanProtocol/const/directoryRoles/[{{generationDirectory}}]",
  "j"
 ],
 [
  "/$defs/SelectedPlanProtocol/const/directoryRoles/[{{recipe.cache}}]",
  "bun-cache"
 ],
 [
  "/$defs/SelectedPlanProtocol/const/directoryRoles/[{{recipe.global}}]",
  "bun-global"
 ],
 [
  "/$defs/SelectedPlanProtocol/const/directoryRoles/[{{recipe.bin}}]",
  "bun-bin"
 ],
 [
  "/$defs/SelectedPlanProtocol/const/directoryRoles/[{{recipe.home}}]",
  "bun-home"
 ],
 [
  "/$defs/SelectedPlanProtocol/const/directoryRoles/[{{recipe.config}}]",
  "bun-config"
 ],
 [
  "/$defs/SelectedRetirementProtocol/const/fileRoles/[{{lockFile}}]",
  "bun.lock"
 ],
 [
  "/$defs/SelectedRetirementProtocol/const/fileRoles/[{{scopeConfig}}]",
  "bunfig.toml"
 ],
 [
  "/$defs/ProducerStageProtocol/const/rootRole",
  "j"
 ],
 [
  "/$defs/ProducerStageProtocol/const/fileRoles/[{{lockFile}}]",
  "bun.lock"
 ],
 [
  "/$defs/ProducerStageProtocol/const/fileRoles/[{{scopeConfig}}]",
  "bunfig.toml"
 ],
 [
  "/$defs/ProducerStageFile/properties/path/pattern",
  "^(package\\.json|bun\\.lock|bunfig\\.toml|patches/(0|[1-9][0-9]*)\\.patch|w/(0|[1-9][0-9]*)/package\\.json)$"
 ],
 [
  "/$defs/SourceOwnerLockProtocol/const/mutableStagePath",
  "bun.lock"
 ]
]
