import re,subprocess,os,sys
R="/Users/ueli/Documents/semio"; os.chdir(R)
files=[f for f in subprocess.check_output(['git','ls-files','-z']).decode().split('\0') if f]
GEN2D="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/📦️packages/🦀️rust/Cargo.toml"
projdep=[l for l in open(GEN2D,encoding='utf-8').read().split('\n') if l.startswith('semio-framework-value-snapshot-projection')][0]
PROJ='''
impl semio_framework_value_snapshot_projection::SnapshotProjection for %(S)s {
    type Output = Self;
    type Projection = semio_framework_value_snapshot_projection::snapshot_projection::OwnedFieldProjection<Self>;
    fn projection_birth_demand() -> semio_framework_value::RetirementDemand { Self::Projection::birth_demand() }
    fn begin_projection(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Self::Projection, semio_framework_value::retained_clone::RetainedCloneProgress), semio_framework_value::ValueError> { Self::Projection::begin(grant) }
}
'''
def rd(p): return open(p,encoding='utf-8').read()
def wr(p,s): open(p,'w',encoding='utf-8').write(s)
def add_clone(s,struct):
    m=re.search(r'(#\[derive\()([^\n]*)(\)\]\n(?:#\[[^\n]*\]\n|///[^\n]*\n)*pub struct '+struct+r'\b)',s)
    assert m,struct
    if 'RetainedClone' in m.group(2): return s
    return s[:m.start()]+m.group(1)+m.group(2)+', semio_framework_value::RetainedClone'+m.group(3)+s[m.end():]
def find(pred):
    m=[f for f in files if pred(f)]; assert len(m)==1,(len(m),m[:2]); return m[0]
def common(artifact,state,assigns):
    st=find(lambda f:f.endswith('✏️editor/🫧️transient/🦀️.rs') and '🀄️wfc/🗿️artifacts/'+artifact+'/' in f)
    d=os.path.dirname(st)
    s=rd(st)
    for t in [state]+assigns: s=add_clone(s,t)
    assert 'SnapshotProjection for' not in s
    s=s.rstrip('\n')+'\n'+PROJ%{'S':state}
    wr(st,s)
    pk=d+'/📦️packages/🦀️rust/Cargo.toml'; p=rd(pk)
    if 'snapshot-projection' not in p: wr(pk,p.replace('semio-framework-value = {',projdep+'\nsemio-framework-value = {',1))
    return st,d
def wfc(artifact,N):
    S=N+'Transient'; D=S+'Diff'; A=S+'Mutation'
    st,d=common(artifact,S,[N+'Assignment'])
    ag=d+'/🧬️schema/🧬️mutations/🦀️.rs'
    leaf=find(lambda f:f.startswith(d+'/🧬️schema/🧬️mutations/') and f.endswith('set-solve/🦀️.rs'))
    s=rd(ag)
    s=s.replace('reference_diff = %s)]'%D,'reference_diff = %s, retained)]'%D,1)
    marker='impl protocol::DiffAlgebra<%s> for %s {'%(S,D); assert marker in s
    s=s.replace(marker,'protocol::retained_row_patch!(pub retained_fields for %s => %s { assignments: Vec<%sAssignment>, contradiction: bool });\nprotocol::retained_patch_diff!(%s => %s);\n\n'%(D,S,N,D,S)+marker,1)
    wr(ag,s)
    l=rd(leaf)
    m=re.search(r'(#\[derive\()([^\n]*)(\)\]\n(?:#\[[^\n]*\]\n)*pub struct SetSolve\b)',l); assert m
    l=l[:m.start()]+m.group(1)+m.group(2)+', semio_framework_value::RetainedClone'+m.group(3)+l[m.end():]
    l=l.rstrip('\n')+'\n\nprotocol::retained_set_record!(SetSolve => %s, %s, %s { authored: SetSolve = |leaf| leaf, patch: |value: SetSolve| %s { assignments: Some(value.assignments), contradiction: Some(value.contradiction) }, inverse: |root: %s| SetSolve { assignments: root.assignments, contradiction: root.contradiction }.into() });\n'%(S,D,A,D,S)
    wr(leaf,l)


def bitmap():
    S,D,A='BitmapTransient','BitmapTransientDiff','BitmapTransientMutation'
    st,d=common('🖼️bitmap',S,[])
    s=rd(st)
    a=s.index("/// 🖼️ The optional solved pixels set to a value or cleared."); b=s.index("/// 🔺️ Field-sparse diff")
    s=s[:a]+s[b:]
    s=s.replace("pub output_pixels: Option<BitmapTransientText>,","pub output_pixels: Option<Option<String>>,",1)
    s=s.replace("output_pixels: self.output_pixels.as_ref().map(|_| BitmapTransientText { value: base.output_pixels.clone() }),","output_pixels: self.output_pixels.as_ref().map(|_| base.output_pixels.clone()),",1)
    s=s.replace("next.output_pixels.clone_from(&value.value);","next.output_pixels.clone_from(value);",1)
    m="impl protocol::DiffAlgebra<BitmapTransient> for BitmapTransientDiff {"; assert m in s
    s=s.replace(m,"protocol::retained_row_patch!(pub retained_fields for BitmapTransientDiff => BitmapTransient { output_pixels: Option<String>, contradiction: bool, output_width: u32, output_height: u32 });\nprotocol::retained_patch_diff!(BitmapTransientDiff => BitmapTransient);\n\n"+m,1)
    assert 'BitmapTransientText' not in s
    wr(st,s)
    ag=d+'/🧬️schema/🧬️mutations/🦀️.rs'; s=rd(ag)
    s=s.replace("BitmapTransientDiff, BitmapTransientText}","BitmapTransientDiff}").replace("reference_diff = BitmapTransientDiff)]","reference_diff = BitmapTransientDiff, retained)]",1)
    assert 'BitmapTransientText' not in s; wr(ag,s)
    leaf=find(lambda f:f.startswith(d+'/🧬️schema/🧬️mutations/') and f.endswith('set-solve/🦀️.rs')); l=rd(leaf)
    l=l.replace("BitmapTransientDiff, BitmapTransientText}","BitmapTransientDiff}")
    l=l.replace("BitmapTransientText { value: self.output_pixels.clone() }","self.output_pixels.clone()")
    assert 'BitmapTransientText' not in l
    m=re.search(r'(#\[derive\()([^\n]*)(\)\]\n(?:#\[[^\n]*\]\n)*pub struct SetSolve\b)',l); assert m
    l=l[:m.start()]+m.group(1)+m.group(2)+', semio_framework_value::RetainedClone'+m.group(3)+l[m.end():]
    l=l.rstrip('\n')+'\n\nprotocol::retained_set_record!(SetSolve => BitmapTransient, BitmapTransientDiff, BitmapTransientMutation { authored: SetSolve = |leaf| leaf, patch: |value: SetSolve| BitmapTransientDiff { output_pixels: Some(value.output_pixels), contradiction: Some(value.contradiction), output_width: Some(value.output_width), output_height: Some(value.output_height) }, inverse: |root: BitmapTransient| SetSolve { output_pixels: root.output_pixels, contradiction: root.contradiction, output_width: root.output_width, output_height: root.output_height }.into() });\n'
    wr(leaf,l)
bitmap()
