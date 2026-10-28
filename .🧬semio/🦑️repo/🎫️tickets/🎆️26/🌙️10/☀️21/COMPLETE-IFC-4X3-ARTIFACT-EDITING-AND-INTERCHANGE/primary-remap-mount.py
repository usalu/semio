from pathlib import Path
home = Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
def update(path, replacements):
    data = path.read_text(encoding='utf-8')
    for old, new, count in replacements:
        assert data.count(old) == count, (path.name, old[:100], data.count(old))
        data = data.replace(old, new)
    result = data.encode('utf-8')
    old_size = path.stat().st_size
    if len(result) < old_size:
        result += b' ' * (old_size - len(result))
    with path.open('r+b') as output:
        output.write(result)
update(home/'work.rs',[
('self.constituent_roster.demands(self.source.as_ref(),&self.parts_cursor,input.snapshot,body)', 'self.constituent_roster.demands(self.source.as_ref(),&self.parts_cursor,&self.value_cursor,input.snapshot,body)',1),
('&mut self.parts_cursor,&mut self.parts,&mut self.pending,cx)', '&mut self.parts_cursor,&mut self.parts,&mut self.pending,&mut self.value_cursor,&mut self.value,cx)',1)])
for name in ['constituent_move_tests.rs','constituent_zero_slot_type_tests.rs']:
    path=home/name
    data=path.read_text(encoding='utf-8')
    replacements=[('&mut work.parts, &mut work.pending, cx)', '&mut work.parts, &mut work.pending, &mut work.value_cursor, &mut work.value, cx)',1)]
    if name=='constituent_move_tests.rs':
        replacements.append(('if variant == 3 { &mut alternate_pending } else { &mut work.pending },\n                        cx,','if variant == 3 { &mut alternate_pending } else { &mut work.pending },\n                        &mut work.value_cursor,\n                        &mut work.value,\n                        cx,',1))
    update(path,replacements)
update(home/'constituent_roster.rs',[
('''            let step = if self.state.phase == 19 {''','''            let child_grant = RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes.min(body.saturating_sub(Self::base().copy_bytes)), ..cx.retained_grant() };
            let step = if self.state.phase == 19 {''',1),
('body.saturating_sub(Self::base().copy_bytes), cx.retained_grant())\n            } else { value_cursor.close_step(cx.retained_grant()) }','body.saturating_sub(Self::base().copy_bytes), child_grant)\n            } else { value_cursor.close_step(child_grant) }',1)])
