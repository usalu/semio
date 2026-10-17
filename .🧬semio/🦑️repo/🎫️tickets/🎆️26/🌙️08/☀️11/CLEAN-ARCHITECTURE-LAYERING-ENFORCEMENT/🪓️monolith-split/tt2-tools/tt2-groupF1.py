from tt2_edit import *
T=ROOT+'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧪️tests/🧪️stdio-xml-valid-conformance-mirror/🟦️.ts'
edit(T,[
 ('dependencies: any,','dependencies: typeof import("../../🟦️.ts"),'),
 ('const without = {','const without: typeof base = {'),
 ('const boundary = {','const boundary: typeof base = {'),
 ("const position = { ...base, doc: { ...base.doc, doctype: { ...base.doc.doctype, prologPosition: 1n } } };","const position: typeof base = { ...base, doc: { ...base.doc, doctype: { name: 'html', declarations: [], prologPosition: 1n } } };"),
 ('const version = {','const version: typeof base = {'),
 ('const encoding = {','const encoding: typeof base = {'),
 ('const good = {','const good: typeof base = {'),
],min_age=0)
