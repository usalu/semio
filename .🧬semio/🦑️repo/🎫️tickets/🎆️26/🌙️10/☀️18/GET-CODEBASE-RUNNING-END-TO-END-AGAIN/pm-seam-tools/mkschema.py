#!/usr/bin/env python3
"""🧬️ Writes the schema-first contract of the package manager seam (draft-07 subset every validator of the repo accepts)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import paths
ident = {"type": "string", "pattern": "^[a-z][a-z0-9-]{0,31}$"}
name = {"$ref": "#/$defs/Name"}
strs = {"type": "array", "items": {"type": "string", "minLength": 1, "maxLength": 256}}
def obj(props, required=None):
    return {"type": "object", "additionalProperties": False, "required": list(props) if required is None else required, "properties": props}
schema = {
 "$schema": "http://json-schema.org/draft-07/schema#",
 "description": "Closed ecosystem-neutral package manager seam: a descriptor declares everything one manager contributes (lock names, configuration, install key recipe, linking backends, operations, installed layout); neutral code only renders, keys, seals and walks through it.",
 "$defs": {
  "Protocol": {"const": {
    "version": 1, "purpose": "package-manager-seam",
    "operations": ["lock", "sync", "verify", "provision", "install-pinned"], "backendKinds": ["clone", "copy", "alias"], "refusedBackendKinds": ["alias"],
    "nameTransforms": ["identity", "lowercase", "go-escape", "pep-escape"], "matches": ["exact", "listed"],
    "lockRoles": ["primary", "secondary"], "configurationRoles": ["scope", "home", "config"],
    "keyShape": ["recipe", "root", "runtime", "platform", "arch", "mode", "members", "material"],
    "bootstrapKeyShape": ["fields", "runtime", "platform", "arch", "lock"],
    "receiptPurpose": "dependency-generation-receipt", "generationNameUnits": 20,
    "selectionEnvironment": "SEMIO_PACKAGE_MANAGER_{ECOSYSTEM}",
    "maximumLockBytes": 67108864, "maximumProviders": 1048576, "maximumMissingReported": 64, "progressInterval": 256}},
  "Identifier": ident,
  "Sha256": {"type": "string", "pattern": "^[a-f0-9]{64}$"},
  "Name": {"type": "string", "minLength": 1, "maxLength": 256},
  "Template": {"type": "string", "minLength": 1, "maxLength": 256},
  "Recipe": {"type": "object", "required": ["version", "kind", "backends", "clearEnvironmentPrefixes", "clearEnvironmentKeys", "cache", "global", "bin", "temporary", "home", "config", "directoryRoles", "maximumEnvironmentEntries", "maximumEnvironmentBytes", "maximumEnvironmentValueUnits", "borrowedProviders"],
             "properties": {"version": {"type": "integer", "minimum": 1}, "kind": ident, "backends": {"type": "object", "additionalProperties": {"type": "string"}},
                            "clearEnvironmentPrefixes": strs, "clearEnvironmentKeys": strs,
                            "cache": name, "global": name, "bin": name, "temporary": name, "home": name, "config": name}},
  "Argument": {"anyOf": [{"$ref": "#/$defs/Template"}, obj({"optional": {"type": "array", "minItems": 1, "items": {"$ref": "#/$defs/Template"}}}), obj({"each": ident, "arguments": {"type": "array", "minItems": 1, "items": {"$ref": "#/$defs/Template"}}})]},
  "Operation": obj({"arguments": {"type": "array", "minItems": 1, "items": {"$ref": "#/$defs/Argument"}}, "scopes": {"type": "object", "additionalProperties": {"type": "array", "items": {"$ref": "#/$defs/Template"}}}, "values": {"type": "object", "additionalProperties": {"type": "string"}}}, ["arguments"]),
  "Descriptor": obj({
    "version": {"const": 1}, "id": ident, "label": name, "ecosystem": ident, "toolchain": ident, "executable": name,
    "runtime": obj({"hostVersionKey": {"type": ["string", "null"]}, "probe": obj({"arguments": strs, "pattern": {"type": "string", "minLength": 1}})}),
    "manifests": {"type": "array", "minItems": 1, "items": name},
    "manifestIdentity": obj({"excludedTopLevelFields": strs, "bootstrapFields": strs}),
    "locks": {"type": "array", "minItems": 1, "items": obj({"name": name, "role": {"enum": ["primary", "secondary"]}, "format": ident})},
    "configurations": obj({"scope": strs, "home": strs, "config": strs}),
    "environment": obj({"set": {"type": "object", "additionalProperties": {"type": "string", "pattern": "^(role|literal):.+$"}}}),
    "recipe": {"$ref": "#/$defs/Recipe"},
    "backendKinds": {"type": "object", "additionalProperties": {"enum": ["clone", "copy", "alias"]}},
    "operations": {"type": "object", "additionalProperties": False, "required": ["lock", "sync"],
                   "properties": {"lock": {"$ref": "#/$defs/Operation"}, "sync": {"$ref": "#/$defs/Operation"}, "verify": {"$ref": "#/$defs/Operation"}, "provision": {"$ref": "#/$defs/Operation"}, "install-pinned": {"$ref": "#/$defs/Operation"}}},
    "installed": obj({"containers": {"type": "array", "minItems": 1, "items": name}, "entry": {"$ref": "#/$defs/Template"}, "match": {"enum": ["exact", "listed"]},
                      "nameTransform": {"enum": ["identity", "lowercase", "go-escape", "pep-escape"]}, "nameEscape": {"type": "object", "additionalProperties": {"type": "string"}}, "fold": {"enum": ["identity", "lowercase", "go-escape", "pep-escape"]}, "suffix": {"type": ["string", "null"]}, "sourcePrefix": {"type": ["string", "null"]}}),
    "resolution": obj({"profiles": strs, "strictPrefix": {"type": ["string", "null"]}, "oracle": name, "conditions": {"type": "object", "additionalProperties": strs}}),
    "ownedModules": strs, "stagedConfiguration": {"type": "string"}, "producerInputs": obj({"scopeFiles": strs, "ambientHomeFiles": strs, "ambientConfigFiles": strs, "ambientConfigFallback": name, "environmentPrefixes": strs, "environmentIgnored": strs}),
    "generationDirectory": ident, "graphComponent": {"type": ["string", "null"]}}),
  "Selection": obj({"version": {"const": 1}, "selection": {"type": "object", "additionalProperties": ident}}),
  "Request": obj({"platform": name, "cache": name, "directory": name, "manifest": name, "scope": name, "lists": {"type": "object", "additionalProperties": {"type": "array", "items": {"type": "string"}}}, "settings": {"type": "object", "additionalProperties": {"type": "string"}}}, ["platform"]),
  "Invocation": obj({"executable": name, "arguments": {"type": "array", "items": {"type": "string"}}, "directory": {"type": ["string", "null"]}}),
  "LockPackage": obj({"name": name, "version": {"type": "string"}, "key": {"type": "string"}, "source": {"type": ["string", "null"]}, "integrity": {"type": ["string", "null"]}}),
  "LockMember": obj({"name": name, "path": {"type": ["string", "null"]}}),
  "LockGraph": obj({"version": {"const": 1}, "format": ident, "packages": {"type": "array", "items": {"$ref": "#/$defs/LockPackage"}}, "members": {"type": "array", "items": {"$ref": "#/$defs/LockMember"}}}),
  "Receipt": obj({
    "version": {"const": 1}, "purpose": {"const": "dependency-generation-receipt"}, "ecosystem": ident,
    "manager": obj({"id": ident, "runtime": {"type": "string", "minLength": 1}}), "scope": name,
    "generation": obj({"key": {"$ref": "#/$defs/Sha256"}, "location": {"type": "string", "pattern": "^[a-z][a-z0-9-]{0,31}/[a-f0-9]{20}$"}}),
    "lock": obj({"name": name, "sha256": {"$ref": "#/$defs/Sha256"}}), "providers": obj({"examined": {"type": "integer", "minimum": 0}}), "sha256": {"$ref": "#/$defs/Sha256"}}),
 }}
text = json.dumps(schema, ensure_ascii=False, indent=2) + "\n"
paths.seam_write(paths.MANAGERS + "/🧬️schema/🔣️.json", text)
print("schema", len(text))
