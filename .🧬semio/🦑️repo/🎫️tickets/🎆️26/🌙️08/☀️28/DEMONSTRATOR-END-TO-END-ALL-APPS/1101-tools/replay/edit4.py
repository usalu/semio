D='/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/'
def sub(path,pairs):
    s=open(D+path).read()
    for a,b,n in pairs:
        assert s.count(a)==n,(path,a[:70],s.count(a))
        s=s.replace(a,b)
    open(D+path,'w').write(s)
sub('🧬️native/🔌️drivers/🦀️.rs',[
('"semantic projection cursor absent"))?.advance(s,body,g)}','"semantic projection cursor absent"))?.advance(s,body,g).result}',1),
('ok_or_else(||invalid("semantic projection cursor absent")).result?.take(child)?else','ok_or_else(||invalid("semantic projection cursor absent"))?.take(child).result?else',1),
('Some(c)if !c.terminal_is_empty()=>c.close_step(g),','Some(c)if !c.terminal_is_empty()=>c.close_step(g).result,',2),
('"semantic retained clone cursor absent"))?.advance(s,body,g)}','"semantic retained clone cursor absent"))?.advance(s,body,g).result}',1),
('Ok(self.cursor.as_mut().ok_or_else(||invalid("semantic retained clone cursor absent"))?.take().result.map(|p|(p,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}','self.cursor.as_mut().ok_or_else(||invalid("semantic retained clone cursor absent"))?.take(g).result}',1),
('let s=f.step(g)?;if f.terminal_is_empty(){self.factory_close=None;}','let s=f.step(g).result?;if f.terminal_is_empty(){self.factory_close=None;}',1),
])
sub('🧬️native/🔗️input/🦀️.rs',[
('return projection.close_step(grant);}','return projection.close_step(grant).result;}',1),
('return source.close_step(grant);}','return source.close_step(grant).result;}',1),
])
sub('🧬️native/🔗️history/🦀️.rs',[('return root.close_step(grant);}','return root.close_step(grant).result;}',1)])
sub('🧬️native/🦀️.rs',[
('fn project_captured<P>(','fn project_captured<P:Send+Sync+\'static>(',1),
('match OriginalVectorCursor::admit(inverse,grant){','match OriginalVectorCursor::admit(inverse,grant).result{',1),
('self.rows.as_mut().unwrap().step(grant)?.progress()','self.rows.as_mut().unwrap().step(grant).result?.progress()',1),
('return semio_framework_value::close_factory_ticket(&mut self.active,grant);}','return semio_framework_value::close_factory_ticket(&mut self.active,grant).result;}',1),
])
