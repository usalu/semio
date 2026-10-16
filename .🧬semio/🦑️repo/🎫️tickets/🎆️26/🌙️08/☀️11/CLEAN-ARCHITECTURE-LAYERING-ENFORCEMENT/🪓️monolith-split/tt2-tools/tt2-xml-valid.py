import shutil,os,time
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
V="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/"
P=V+"🟦️.ts"; T=V+"🧪️tests/🧪️stdio-xml-valid-conformance-mirror/🟦️.ts"
for p in (P,T): assert time.time()-os.stat(p).st_mtime>1800
shutil.copy(P,F+"/🗑️generated/tt2-snap/xml-valid-schema.ts.pre"); shutil.copy(T,F+"/🗑️generated/tt2-snap/xml-valid-mirror-test.ts.pre")
s=open(P,encoding="utf8").read(); t=open(T,encoding="utf8").read()
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(old,s.count(old)); s=s.replace(old,new)
sub("import type { XmlDocument, XmlNode, XmlSnapshot } from","import type { XmlDocument, XmlDoctype, XmlNode, XmlSnapshot } from")
sub("export const CODE_VALIDITY_NOT_VERIFIED = 'stdio.xml.valid.validity-not-fully-verified';\n","""export const CODE_VALIDITY_NOT_VERIFIED = 'stdio.xml.valid.validity-not-fully-verified';
export const CODE_DOCUMENT_ELEMENT_MISSING = 'stdio.xml.valid.document-element-missing';
export const CODE_BOUNDARY_KIND = 'stdio.xml.valid.boundary-kind';
export const CODE_DOCTYPE_POSITION = 'stdio.xml.valid.doctype-position';
export const CODE_VERSION = 'stdio.xml.valid.version';
export const CODE_ENCODING_NAME = 'stdio.xml.valid.encoding-name';
""")
a=s.index("function parseDoctypeRootName"); b=s.index("function rootElementName")
s=s[:a]+s[b:]
a=s.index("function doctypeReferencesExternalSubset"); b=s.index("/** 🛡️ Scope-limited")
s=s[:a]+"""const hard = (code: string, message: string): XmlValidDiagnostic => ({ code, severity: 'error', message });
const soft = (code: string, message: string): XmlValidDiagnostic => ({ code, severity: 'warning', message });

"""+s[b:]
a=s.index("export function checkValidConformance"); b=s.index("export function snapshotWith")
s=s[:a]+"""export function checkValidConformance(snapshot: XmlSnapshot): XmlValidDiagnostic[] {
  const out: XmlValidDiagnostic[] = [];
  const { doctype, declaration, prolog, epilog } = snapshot.doc;
  const actualRoot = rootElementName(snapshot);
  if (actualRoot === undefined) out.push(hard(CODE_DOCUMENT_ELEMENT_MISSING, 'XML validity requires a document element'));
  if ([...prolog, ...epilog].some((node) => node.kind !== 'comment' && node.kind !== 'processingInstruction')) {
    out.push(hard(CODE_BOUNDARY_KIND, 'XML validity permits only comments and processing instructions outside the document element'));
  }
  if (doctype !== undefined && (doctype.prologPosition ?? 0n) > BigInt(prolog.length)) {
    out.push(hard(CODE_DOCTYPE_POSITION, 'XML validity requires the doctype to occupy its declared prolog position'));
  }
  if (declaration !== undefined) {
    if (declaration.version !== '1.0') out.push(hard(CODE_VERSION, 'XML 1.0 validity requires declaration version 1.0'));
    if (declaration.encoding !== undefined && !/^[A-Za-z][A-Za-z0-9._-]*$/.test(declaration.encoding)) {
      out.push(hard(CODE_ENCODING_NAME, 'XML validity requires a syntactically valid declared encoding name'));
    }
  }
  if (doctype === undefined) {
    out.push(hard(CODE_DOCTYPE_MISSING, 'no <!DOCTYPE ...> declaration present -- XML 1.0 §5.1 validity requires one (a document without one can be well-formed at best)'));
  } else {
    if (actualRoot !== undefined && doctype.name !== actualRoot) {
      out.push(hard(CODE_ROOT_NAME_MISMATCH, `doctype declares root name '${doctype.name}' but the actual root element is '<${actualRoot}>' -- §2.8 requires the DOCTYPE Name to match the document element`));
    }
    if (doctype.externalId !== undefined && declaration?.standalone === true) {
      out.push(soft(CODE_STANDALONE_EXTERNAL_SUBSET, 'XML declaration says standalone="yes" but the doctype references an external subset (SYSTEM/PUBLIC) -- suspicious per §2.9'));
    }
  }
  out.push(soft(CODE_VALIDITY_NOT_VERIFIED, 'validity not fully verified: the typed doctype retains its name, external identifiers and entity declarations; DTD element and attribute content models (§3.2/§3.3) are absent from this owned schema'));
  return out;
}

"""+s[b:]
sub("""  doctype: string | undefined,
  standalone: boolean | undefined,""","""  doctype: Pick<XmlDoctype, 'name' | 'externalId'> | undefined,
  standalone: boolean | undefined,""")
sub("""    doctype,
    root,
  };""","""    doctype: doctype === undefined ? undefined : { ...doctype, declarations: [] },
    root,
    prolog: [],
    epilog: [],
  };""")
sub("export function snapshotWith(","/** 🧪 Build a minimal snapshot around one structured doctype for the conformance mirror tests. */\nexport function snapshotWith(")
sub("""{ CODE_DOCTYPE_MISSING, CODE_ROOT_NAME_MISMATCH, CODE_STANDALONE_EXTERNAL_SUBSET, CODE_VALIDITY_NOT_VERIFIED, checkValidConformance, meta, snapshotWith }, {""","""{ CODE_BOUNDARY_KIND, CODE_DOCTYPE_MISSING, CODE_DOCTYPE_POSITION, CODE_DOCUMENT_ELEMENT_MISSING, CODE_ENCODING_NAME, CODE_ROOT_NAME_MISMATCH, CODE_STANDALONE_EXTERNAL_SUBSET, CODE_VALIDITY_NOT_VERIFIED, CODE_VERSION, checkValidConformance, meta, snapshotWith }, {""")
open(P,"w",encoding="utf8").write(s)
def subt(old,new,count=1):
    global t
    assert t.count(old)==count,(old,t.count(old)); t=t.replace(old,new)
subt("const { CODE_DOCTYPE_MISSING, CODE_ROOT_NAME_MISMATCH, CODE_STANDALONE_EXTERNAL_SUBSET, CODE_VALIDITY_NOT_VERIFIED, checkValidConformance, meta, snapshotWith } = dependencies;","const { CODE_BOUNDARY_KIND, CODE_DOCTYPE_MISSING, CODE_DOCTYPE_POSITION, CODE_DOCUMENT_ELEMENT_MISSING, CODE_ENCODING_NAME, CODE_ROOT_NAME_MISMATCH, CODE_STANDALONE_EXTERNAL_SUBSET, CODE_VALIDITY_NOT_VERIFIED, CODE_VERSION, checkValidConformance, meta, snapshotWith } = dependencies;")
subt("snapshotWith('<!DOCTYPE html>', undefined, 'html')","snapshotWith({ name: 'html' }, undefined, 'html')")
subt("snapshotWith('<!DOCTYPE book>', undefined, 'html')","snapshotWith({ name: 'book' }, undefined, 'html')")
subt("""snapshotWith('<!DOCTYPE html SYSTEM "http://example.com/html.dtd">', true, 'html'),""","""snapshotWith({ name: 'html', externalId: { kind: 'system', systemId: 'http://example.com/html.dtd' } }, true, 'html'),""")
subt("""      expect(diagnostics.some((d) => d.code === CODE_STANDALONE_EXTERNAL_SUBSET)).toBe(true);
    });
""","""      expect(diagnostics.some((d) => d.code === CODE_STANDALONE_EXTERNAL_SUBSET && d.severity === 'warning')).toBe(true);
    });

    it('standalone yes without an external subset stays quiet', () => {
      const diagnostics = checkValidConformance(snapshotWith({ name: 'html' }, true, 'html'));
      expect(diagnostics.map((d) => d.code)).toEqual([CODE_VALIDITY_NOT_VERIFIED]);
    });

    it('a document element, boundary kinds, doctype position, version and encoding are checked like the owned Rust validator', () => {
      const base = snapshotWith({ name: 'html' }, undefined, 'html');
      const without = { ...base, doc: { ...base.doc, root: undefined } };
      expect(checkValidConformance(without).some((d) => d.code === CODE_DOCUMENT_ELEMENT_MISSING && d.severity === 'error')).toBe(true);
      const boundary = { ...base, doc: { ...base.doc, epilog: [{ kind: 'text', text: 'x' }] } };
      expect(checkValidConformance(boundary).some((d) => d.code === CODE_BOUNDARY_KIND)).toBe(true);
      const position = { ...base, doc: { ...base.doc, doctype: { ...base.doc.doctype, prologPosition: 1n } } };
      expect(checkValidConformance(position).some((d) => d.code === CODE_DOCTYPE_POSITION)).toBe(true);
      const version = { ...base, doc: { ...base.doc, declaration: { version: '1.1' } } };
      expect(checkValidConformance(version).some((d) => d.code === CODE_VERSION)).toBe(true);
      const encoding = { ...base, doc: { ...base.doc, declaration: { version: '1.0', encoding: '9bad' } } };
      expect(checkValidConformance(encoding).some((d) => d.code === CODE_ENCODING_NAME)).toBe(true);
      const good = { ...base, doc: { ...base.doc, declaration: { version: '1.0', encoding: 'UTF-8' } } };
      expect(checkValidConformance(good).map((d) => d.code)).toEqual([CODE_VALIDITY_NOT_VERIFIED]);
    });
""")
open(T,"w",encoding="utf8").write(t)
