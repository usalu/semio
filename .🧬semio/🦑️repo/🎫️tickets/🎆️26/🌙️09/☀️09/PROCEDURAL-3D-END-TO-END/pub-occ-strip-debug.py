import re,sys,pathlib
S="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/"
L="🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs"
def edit(path,pairs):
    p=pathlib.Path(path);t=p.read_text();o=t
    for a,b in pairs:
        n=t.count(a)
        if n!=1: print("SKIP",path[-40:],n,a[:60]);continue
        t=t.replace(a,b)
    if t!=o:p.write_text(t);print("EDITED",path[-50:])
edit(L,[('eprintln!("[DEBUG] page allocation occupied caller={} actual={:?}",std::panic::Location::caller(),effect.as_deref_mut().and_then(|effect|*effect.actual()));','')])
blk='    #[cfg(not(test))]\n    { static SEEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false); if !SEEN.swap(true, std::sync::atomic::Ordering::Relaxed) { eprintln!("[DEBUG] %s blocked at {}", std::panic::Location::caller()); } }\n'
edit(S+"🫧️ephemeral/🌫️transient-and-interaction-stores/🦀️.rs",[(blk%"store","")])
edit(S+"👥️presence/👥️presence-store-and-peers-publication/🦀️.rs",[(blk%"store","")])
edit(S+"⚙️engine/🧳️bundle/🦀️.rs",[(blk%"batch",""),('else{eprintln!("[DEBUG] digest step not admitted work={work} grant={:?}",grant.retained_grant());return Ok(blocked_here())}','else{return Ok(blocked_here())}'),('if ownership==Default::default(){eprintln!("[DEBUG] digest zero progress demand={:?} work={work} grant={:?}",owner.next_digest_demand(),grant.retained_grant());ArtifactStoreOneItemAdvance::Blocked}','if ownership==Default::default(){ArtifactStoreOneItemAdvance::Blocked}')])
dg=S+"🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs"
t=pathlib.Path(dg).read_text()
m=re.search(r'  if self\.next_digest_work_demand\(\)\?==0\{static SEEN:.*?\n',t)
print("digest line",bool(m))
if m: pathlib.Path(dg).write_text(t.replace(m.group(0),'',1));print("EDITED digest")
