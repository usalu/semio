# 🔧 Package-form policy: implementation admission for nested build units (Go sub-packages, Cargo crates) plus analyzer fixes. usage: python3 apply-3-package-admission.py [--check]  (run after the library freeze lifts)
import re,shutil,sys,os
check="--check" in sys.argv
R="/Users/ueli/Documents/semio"
import os as _os
L=_os.environ.get("TAX3_L") or f"{R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
C=f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
files={}
def load(p):
    if p not in files: files[p]=open(p,encoding="utf8").read()
    return files[p]
def rep(p,old,new,count=1):
    s=load(p); assert s.count(old)==count,(p[-60:],old[:70],s.count(old)); files[p]=s.replace(old,new)
ADM=f"{L}/🔍️discovery/🧱️package-implementation-admission/🟦️.ts"
# 1 taxonomy.json
T=f"{L}/🔣️taxonomy.json"
s=load(T)
def add_admission(ecosystem,block):
    global s
    i=s.index('"packageBoundaryRules"'); j=s.index(f'"{ecosystem}": {{',i)
    k=s.index('"recursive": true,',j)
    s=s[:k]+block+s[k:]
add_admission("🐹️go",'"implementationAdmission": {\n        "form": "nested-source-unit",\n        "unitSourceFileKindId": "go-source"\n      },\n      ')
add_admission("🦀️rust",'"implementationAdmission": {\n        "form": "nested-manifest-unit",\n        "unitManifestContractId": "cargo-manifest"\n      },\n      ')
files[T]=s
# 2 types
for p,old in [(f"{L}/🔍️discovery/📜️taxonomy-contracts/🟦️.ts",'  readonly implementationRole: "problem";\n}\n\n'),(f"{L}/🧹️normalization/🔣️taxonomy/🏭️generators-packages/🟦️.ts",'  readonly implementationRole: "problem";\n}\n\nexport interface PackageBoundaryProfile')]:
    s=load(p); i=s.index("export interface PackageBoundaryRule"); k=s.index('  readonly implementationRole: "problem";\n',i)
    s=s[:k]+'  readonly implementationRole: "problem";\n  readonly implementationAdmission: PackageImplementationAdmission | null;\n'+s[k+len('  readonly implementationRole: "problem";\n'):]
    imp=('import type { PackageImplementationAdmission } from "../🧱️package-implementation-admission/🟦️.ts";\n' if "discovery/📜" in p else 'import type { PackageImplementationAdmission } from "../../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";\n')
    files[p]=imp+s
# 3 loader
P=f"{L}/🧹️normalization/🔣️taxonomy/🏭️generators-packages-and-exclusions/🟦️.ts"
rep(P,'''      implementationRole: "problem",
    };
    const rule = packageBoundaryRules[id];''','''      implementationRole: "problem",
      implementationAdmission: parseImplementationAdmission(spec.implementationAdmission, `packageBoundaryRules.${id}.implementationAdmission`),
    };
    const rule = packageBoundaryRules[id];''')
rep(P,'import type { SemanticPackageGeneration } from "../../../🔍️discovery/📦️package-projection-contracts/🟦️.ts";\n','import type { SemanticPackageGeneration } from "../../../🔍️discovery/📦️package-projection-contracts/🟦️.ts";\nimport type { PackageImplementationAdmission } from "../../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";\n')
rep(P,'/** ⚙ Parses the generator contracts and the ecosystems. */','''function parseImplementationAdmission(value: unknown, name: string): PackageImplementationAdmission | null {
  if (value === undefined || value === null) return null;
  const row = requireRecord(value, name);
  if (row.form === "nested-source-unit") {
    requireExactKeys(row, ["form", "unitSourceFileKindId"], name);
    return { form: "nested-source-unit", unitSourceFileKindId: requireString(row.unitSourceFileKindId, `${name}.unitSourceFileKindId`) };
  }
  if (row.form === "nested-manifest-unit") {
    requireExactKeys(row, ["form", "unitManifestContractId"], name);
    return { form: "nested-manifest-unit", unitManifestContractId: requireString(row.unitManifestContractId, `${name}.unitManifestContractId`) };
  }
  throw new Error(`Taxonomy v7 ${name}.form must be nested-source-unit or nested-manifest-unit`);
}

/** ⚙ Parses the generator contracts and the ecosystems. */''')
# 4 audit
A=f"{L}/🔍️discovery/🚛️audit-package-boundary-contracts/🟦️.ts"
rep(A,'''    if (rule.recursive !== true || rule.uncertainRole !== "problem" || rule.implementationRole !== "problem") problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] must recursively block uncertain and implementation roles.`);''','''    if (rule.recursive !== true || rule.uncertainRole !== "problem" || rule.implementationRole !== "problem") problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] must recursively block uncertain and implementation roles.`);
    const admission = rule.implementationAdmission;
    if (admission !== null && admission !== undefined) {
      if (admission.form === "nested-source-unit") {
        exactKeys(admission, ["form", "unitSourceFileKindId"], `packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission`);
        if (!rule.allowedFileKindIds.includes(admission.unitSourceFileKindId) || taxonomy.fileKinds[admission.unitSourceFileKindId]?.role !== "source") problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission.unitSourceFileKindId must be an allowed source file kind.`);
      } else if (admission.form === "nested-manifest-unit") {
        exactKeys(admission, ["form", "unitManifestContractId"], `packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission`);
        if (admission.unitManifestContractId !== rule.manifestContractId) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission.unitManifestContractId must be the package manifest contract.`);
      } else problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission.form is invalid.`);
    }''')
# 5 role type
rep(f"{L}/🧹️normalization/📜️plan-contracts/🟦️.ts",'"thin-delegation" | "implementation" | "unresolved" | "not-package";','"thin-delegation" | "implementation" | "admitted-implementation" | "unresolved" | "not-package";')
# 6 classifyPackageRole
F=f"{L}/🧹️normalization/📏️fixed-contracts/🟦️.ts"
rep(F,'export function classifyPackageRole(path: string, kindId: string | null, fixedId: string | undefined, content: string | null, taxonomy: LoadedTaxonomy): TaxonomyPackageRole {','''/** 🧱️ Whether the schema admits implementation at this package file: it sits in a nested build unit of its package rule. `unitHasFile` answers whether a repo path exists as a file. */
export function packageImplementationAdmittedAt(path: string, kindId: string | null, taxonomy: LoadedTaxonomy, unitHasFile: (path: string) => boolean): boolean {
  const location = packageLocation(path, taxonomy);
  if (!location?.rule) return false;
  return packageImplementationAdmitted(location.rule.implementationAdmission, {
    relativePath: path.slice(location.packageRoot.length + 1),
    kindId,
    directoryHasFile: (name) => unitHasFile(`${dirname(path)}/${name}`),
    manifestFilename: (contractId) => { const contract = taxonomy.schema.fixedFilenameContracts[contractId]; return contract ? fixedContractFilename(contract) : null; },
  });
}

export function classifyPackageRole(path: string, kindId: string | null, fixedId: string | undefined, content: string | null, taxonomy: LoadedTaxonomy, admitted = false): TaxonomyPackageRole {''')
rep(F,'''  if (kindId && !location.rule.allowedFileKindIds.includes(kindId)) return "implementation";''','''  if (kindId && !location.rule.allowedFileKindIds.includes(kindId)) return admitted ? "admitted-implementation" : "implementation";''')
rep(F,'''  return grammar.allowedRoles.includes(role as PackageGlueGrammar["allowedRoles"][number]) ? role : role === "implementation" ? "implementation" : "unresolved";''','''  if (role === "implementation" && admitted) return "admitted-implementation";
  return grammar.allowedRoles.includes(role as PackageGlueGrammar["allowedRoles"][number]) ? role : role === "implementation" ? "implementation" : "unresolved";''')
rep(F,'import { classifyPackageSource, classifyPackageSourceDisposition } from "../../🔍️discovery/🏷️package-source-classification/🟦️.ts";\n','import { classifyPackageSource, classifyPackageSourceDisposition } from "../../🔍️discovery/🏷️package-source-classification/🟦️.ts";\nimport { packageImplementationAdmitted } from "../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";\nimport { fixedContractFilename } from "../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts";\n')
# 7 inventory planning
I=f"{L}/🧹️normalization/🔍️inventory-planning/🟦️.ts"
rep(I,'''    const role = packageLocation(row.path, taxonomy) ? storedClassification(leafStore, () => recordKey("taxonomy-package-role-v1", normalizationRevision(), taxonomy.input.contentHash, row.path, canonical.fileKind ?? "", canonical.fixedId ?? "", content.hash, String(textual)), () => classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, text(), taxonomy)) : classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, null, taxonomy);''','''    const admitted = packageImplementationAdmittedAt(row.path, canonical.fileKind, taxonomy, (candidate) => leafPaths.has(candidate));
    const role = packageLocation(row.path, taxonomy) ? storedClassification(leafStore, () => recordKey("taxonomy-package-role-v2", normalizationRevision(), taxonomy.input.contentHash, row.path, canonical.fileKind ?? "", canonical.fixedId ?? "", content.hash, String(textual), String(admitted)), () => classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, text(), taxonomy, admitted)) : classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, null, taxonomy);''')
s=load(I); m=re.search(r'import \{([^}]*)\bclassifyPackageRole\b([^}]*)\} from "\.\./📏️fixed-contracts/🟦️\.ts";',s)
assert m,"classifyPackageRole import"
files[I]=s.replace(m.group(0),m.group(0).replace("classifyPackageRole","classifyPackageRole, packageImplementationAdmittedAt",1))
# 8 discovery scan
S=f"{L}/🔍️discovery/🔭️repository-scan/🟦️.ts"
rep(S,'''        if (decision.role === "implementation") addPackageProblem(owner, path, "package-implementation",''','''        const admitted = decision.role === "implementation" && packageImplementationAdmitted(rule.implementationAdmission, { relativePath: relative(packageRoot, path).split("\\\\").join("/"), kindId, directoryHasFile: (name) => existsSync(join(dirname(path), name)), manifestFilename: (contractId) => { const contract = taxonomy.fixedFilenameContracts[contractId]; return contract ? fixedContractFilename(contract) : null; } });
        if (admitted) continue;
        if (decision.role === "implementation") addPackageProblem(owner, path, "package-implementation",''')
rep(S,'import { semanticPackageGenerationAuthority } from "../🎁️package-projection/🟦️.ts";\n','import { semanticPackageGenerationAuthority } from "../🎁️package-projection/🟦️.ts";\nimport { packageImplementationAdmitted } from "../🧱️package-implementation-admission/🟦️.ts";\n')
# 9 classifier fixes
K=f"{L}/🔍️discovery/🏷️package-source-classification/🟦️.ts"
rep(K,'''    if (char === '"' || char === "'" || char === "`") {
      const quote = char;
      let end = index + 1, escaped = false;
      for (; end < content.length; end++) {
        const value = content[end]!;
        if (!escaped && value === quote) { end++; break; }
        escaped = !escaped && value === "\\\\";
        if (value !== "\\\\") escaped = false;
      }''','''    if (char === '"' || char === "'" || char === "`") {
      const quote = char, raw = quote === "`" && analyzer === "go";
      let end = index + 1, escaped = false;
      for (; end < content.length; end++) {
        const value = content[end]!;
        if (!escaped && value === quote) { end++; break; }
        escaped = !raw && !escaped && value === "\\\\";
        if (value !== "\\\\") escaped = false;
      }''')
rep(K,'''    .replace(/(?:pub(?:\\([^)]*\\))?\\s+)?(?:unsafe\\s+)?extern\\s+crate\\s+[^;]+;/gu, "")
    .replace(/(?:pub(?:\\([^)]*\\))?\\s+)?use\\s+[^;]+;/gu, "")
    .replace(/(?:pub(?:\\([^)]*\\))?\\s+)?(?:mod|type)\\s+[^;{}]+;/gu, "")''','''    .replace(/\\b(?:pub(?:\\([^)]*\\))?\\s+)?(?:unsafe\\s+)?extern\\s+crate\\s+[^;]+;/gu, "")
    .replace(/\\b(?:pub(?:\\([^)]*\\))?\\s+)?use\\s+[^;]+;/gu, "")
    .replace(/\\b(?:pub(?:\\([^)]*\\))?\\s+)?(?:mod|type)\\s+[^;{}]+;/gu, "")''')
rep(K,'''    rest = rest.replace(/(?:pub\\s+)?mod\\s+[A-Za-z_]\\w*\\s*\\{(?:\\s*(?:(?:pub\\s+)?(?:use|mod|type)\\s+[^;{}]+;|#!?\\[[^\\]]*\\]))*\\s*\\}/gu, "").trim();''','''    rest = rest.replace(/\\b(?:pub\\s+)?mod\\s+[A-Za-z_]\\w*\\s*\\{(?:\\s*(?:\\b(?:pub\\s+)?(?:use|mod|type)\\s+[^;{}]+;|#!?\\[[^\\]]*\\]))*\\s*\\}/gu, "").trim();''')
rep(K,'''    .replace(/\\bimport\\s*(?:\\([^)]*\\)|"[^"]*"|'[^']*')/gu, "")
    .trim();
  if (/\\b(?:type|var|const)\\b/u.test(rest)) return { role: "implementation", evidence: "Go runtime declaration remains" };''','''    .replace(/\\bimport\\s*(?:[A-Za-z_.]\\w*\\s*)?(?:\\([^)]*\\)|"[^"]*"|'[^']*')/gu, "")
    .trim();
  if (/\\b(?:func|type|var|const)\\b/u.test(rest)) return { role: "implementation", evidence: "Go method, type or runtime declaration remains" };''')
# write
for p,t in files.items():
    if check: print("ok",p[-70:]); continue
    open(p,"w",encoding="utf8").write(t)
if not check:
    os.makedirs(os.path.dirname(ADM),exist_ok=True); shutil.copy(f"{C}/tax3-tools/new/package-implementation-admission.ts",ADM)
print("done" if not check else "check ok")
