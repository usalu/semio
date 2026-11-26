import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { join } from "node:path";
import law from "./stdio-role-carve26.json" with {type:"json"};

const standard = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw";
const registry = join(standard,"📦️packages/🦀️rust");
const mutations = join(standard,"🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust");
const text = (root:string,file:string) => Bun.file(join(root,file)).text();

test("Binary keeps its two surfaces and pays for editor closed dispatch in the existing editor owner",async()=>{
 const [outer,owner,surface,registryToml,mutationsToml]=await Promise.all([
  text(registry,"🦀️.rs"),text(mutations,"🦀️.rs"),text(standard,"🪆️subsets/✳️any/🦀️.rs"),
  text(registry,"Cargo.toml"),text(mutations,"Cargo.toml"),
 ]);
 const registryManifest=Bun.TOML.parse(registryToml) as {package:{name:string};dependencies:Record<string,unknown>};
 const editorManifest=Bun.TOML.parse(mutationsToml) as {package:{name:string};dependencies:Record<string,unknown>};
 expect(registryManifest.package.name).toBe(law.registry);
 expect(editorManifest.package.name).toBe(law.editorOwner);
 expect(registryManifest.dependencies[law.editorOwner]).toBeDefined();
 expect(editorManifest.dependencies["semio-framework-dispatch-macros"]).toBeDefined();
 expect(editorManifest.dependencies["semio-plugin"]).toBeDefined();
 const actualOuter=/pub enum BinaryApps: PluginApp\s*\{([^}]+)\}/u.exec(outer)?.[1];
 const actualEditor=/pub enum BinaryEditorApps: PluginApp\s*\{([^}]+)\}/u.exec(owner)?.[1];
 const sql=new Database(":memory:");
 try{
  sql.exec("CREATE TABLE expected (app TEXT, role TEXT, payload TEXT); CREATE TABLE actual (app TEXT, role TEXT, payload TEXT);");
  const expected=sql.query("INSERT INTO expected VALUES (?1,?2,?3)");
  expected.run(law.apps,law.editor.role,law.editor.closed);
  expected.run(law.apps,law.viewer.role,law.viewer.concrete);
  expected.run(law.editor.closed,law.editor.role,law.editor.payload);
  const actual=sql.query("INSERT INTO actual VALUES (?1,?2,?3)");
  for(const [app,body] of [[law.apps,actualOuter],[law.editor.closed,actualEditor]] as const){
   for(const row of (body??"").matchAll(/(\w+)\((.+)\),/gu))actual.run(app,row[1]!,row[2]!);
  }
  const difference=sql.query("SELECT * FROM (SELECT * FROM expected EXCEPT SELECT * FROM actual) UNION ALL SELECT * FROM (SELECT * FROM actual EXCEPT SELECT * FROM expected)").all();
  expect(difference).toEqual([]);
 }finally{sql.close();}
 expect(owner).toContain(`pub type ${law.editor.payload} = ${law.editor.concrete};`);
 expect(outer).toMatch(/impl From<BinaryEditorApp> for BinaryApps\s*\{\s*fn from\(app: BinaryEditorApp\) -> Self\s*\{\s*Self::Editor\(app.into\(\)\)\s*\}\s*\}/u);
 expect(surface).toContain("editor_surface::<editor::BinaryEditor, crate::BinaryApps>(editor::create_binary_editor())");
 expect(surface).toContain("viewer_surface::<viewer::BinaryViewer, crate::BinaryApps>(viewer::create_binary_viewer())");
 expect(surface).toContain(`artifact_kind: "${law.artifact}"`);
});
