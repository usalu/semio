p="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts"
t=open(p,encoding="utf8").read()
def rep(old,new):
    global t
    assert t.count(old)==1,(t.count(old),old)
    t=t.replace(old,new)
rep('''  const host=ownerSource(owner);
  const initializer=ownerSlice(owner,"struct DrawingStoreInitializationAuthority","pub fn drawing_document_store_initialization_job");
  expect(initializer.includes("drop(self.owner_catalog.take())")).toBe(false);
  expect(initializer.includes("close_initialization_catalog(&mut self.owner_catalog, 1, maximum_bytes)")).toBe(true);
  expect(initializer.includes("close_initialization_catalog(&mut self.owner_catalog, maximum_items, maximum_bytes)")).toBe(true);
  expect(initializer.includes("PluginCloseStep::Pending { released_items, released_bytes }")).toBe(true);
  const source=readFileSync(join(directory,"🦀️.rs"),"utf8");
  expect(source.includes("catalog.close_step(1, maximum_bytes)")).toBe(true);
''','''  const initializer=ownerSlice(owner,"struct DrawingStoreInitializationAuthority","pub fn drawing_document_store_initialization_job");
  expect(initializer.includes("drop(self.owner_catalog.take())")).toBe(false);
  expect(initializer.includes("let child = RetainedCloneGrant { maximum_items: 1,")).toBe(true);
  expect(initializer.includes("close_initialization_catalog(&mut self.owner_catalog, child)")).toBe(true);
  expect(initializer.includes("RetainedCloneStep::Progress(step.progress())")).toBe(true);
  const source=readFileSync(join(directory,"🦀️.rs"),"utf8");
  expect(source.includes("catalog.close_step(grant)")).toBe(true);
''')
rep('''  const host=ownerSource(owner);
  const initializer=ownerSlice(owner,"struct DrawingStoreInitializationAuthority","pub fn drawing_document_store_initialization_job");
  expect(initializer.includes("active: std::mem::ManuallyDrop<Option<DrawingOwnedRetirement>>")).toBe(true);
''','''  const initializer=ownerSlice(owner,"struct DrawingStoreInitializationAuthority","pub fn drawing_document_store_initialization_job");
  expect(initializer.includes("retire_pending: std::mem::ManuallyDrop<Option<DrawingRetirementOwner>>")).toBe(true);
  for(const kind of law.initializerOwners)expect(initializer.includes(`*self.retire_pending = Some(DrawingRetirementOwner::${kind}(`)).toBe(true);
  expect(initializer.includes("artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.active, child)")).toBe(true);
''')
rep('''  expect(source.includes("DrawingDecodedFieldRetirement<DrawingRetirementOwner>")).toBe(true);
  expect(source.includes("owned_retirement(")).toBe(false);
  expect(source.includes("self.owner.next_grant()")).toBe(true);
  const helper=ownerSource(owner);
  for(const axis of law.axes)expect(helper.includes(`next_${axis}_byte_demand`)).toBe(true);
''','''  expect(source.includes("#[derive(semio_framework_value::RetireOwned)]")).toBe(true);
  expect(source.includes("pub enum DrawingRetirementOwner")).toBe(true);
  for(const variant of law.variants)expect(source.includes(`    ${variant}(`)).toBe(true);
  expect(source.includes("owned_retirement(")).toBe(false);
  const helper=ownerSource(owner);
  expect(helper.includes("pub struct DrawingDecodedFieldRetirement<T: semio_framework_value::retirement::RetireOwned>")).toBe(true);
  expect(helper.includes("pub fn next_grant(&self)")).toBe(true);
  for(const axis of law.axes)expect(helper.includes(`next_${axis}_byte_demand`)).toBe(true);
''')
rep('''      const bytes=row.kind==="text"?new TextEncoder().encode(row.value).length:row.kind==="u32-list"?Uint32Array.from(row.value).byteLength:Uint32Array.of(row.value).byteLength;
      expect(bytes).toBe(row.totalCopyBytes);
      const moved=patched({field:{value:row.value,backingBytes:128}}''','''      const bytes=row.kind==="u32-list"?Uint32Array.from(row.value).byteLength:0;
      expect(bytes).toBe(row.totalCopyBytes);
      const moved=patched({field:{value:row.value,backingBytes:128}}''')
rep('''  const rewrite=ownerSlice(owner,"impl DrawingDuplicateRewriteAuthority","enum DrawingMutationCandidatePhase");
  expect(rewrite.includes("fn close_granted(")).toBe(true);''','''  const rewrite=ownerSlice(owner,"impl DrawingDuplicateRewriteAuthority");
  expect(rewrite.includes("pub fn close_step(&mut self, grant: RetainedCloneGrant)")).toBe(true);''')
rep('''  expect(authorities.includes("DrawingDecodedFieldRetirement<$value>")).toBe(true);''','''  expect(authorities.includes("artifact_retirement_owned_birth_demands(&self.value)")).toBe(true);
  expect(authorities.includes("artifact_retirement_box_demands(owner, maximum_copy_bytes)")).toBe(true);
  expect(authorities.includes("artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant)")).toBe(true);''')
rep('pub path_segments: Option<PagedList<crate::PathSegment, {usize::MAX}>>','pub path_segments: Option<PagedList<semio_s_artifact_draw_drawing_model::PathSegment, {usize::MAX}>>')
rep('''  expect(layerClone.includes("close_granted")).toBe(true);
  expect(layerClone.includes("Vec::with_capacity")).toBe(false);''','''  expect(layerClone.includes("pub fn close_step(&mut self,grant:RetainedCloneGrant)")).toBe(true);
  expect(layerClone.includes("Vec::with_capacity")).toBe(false);''')
open(p,"w",encoding="utf8").write(t)
