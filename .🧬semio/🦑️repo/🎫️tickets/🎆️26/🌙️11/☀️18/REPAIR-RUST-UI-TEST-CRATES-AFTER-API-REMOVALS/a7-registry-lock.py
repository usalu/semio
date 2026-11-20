import sys
p=sys.argv[1];s=open(p,encoding='utf-8').read()
def r(a,b):
    global s
    assert s.count(a)==1,a[:80];s=s.replace(a,b)
r("use std::{ops::{Deref,DerefMut},sync::{MutexGuard,TryLockError}};","use std::{cell::UnsafeCell,marker::PhantomData,ops::{Deref,DerefMut},sync::atomic::{AtomicBool,Ordering}};")
r("""/// 🛡️ The mutex protects both the unique publication pointer and each live borrowed arena access.
pub struct UiArenaRegistry{state:Mutex<RegisteredArena>}
impl UiArenaRegistry{
 pub const fn new()->Self{Self{state:Mutex::new(RegisteredArena{pointer:0,epoch:0,closing:false})}}
 pub fn borrow(&'static self)->Result<Option<UiArenaBorrow>,ValueError>{let guard=match self.state.try_lock(){Ok(guard)=>guard,Err(TryLockError::WouldBlock)=>return Ok(None),Err(TryLockError::Poisoned(_))=>return Err(invalid())};if guard.pointer==0||guard.closing{""",
"""/// 🛡️ One allocation-free poisoning lock protects both the unique publication pointer and each live borrowed arena access, so the resident static registry never lazily boxes a platform mutex outside any receipt.
pub struct UiArenaRegistry{held:AtomicBool,poisoned:AtomicBool,state:UnsafeCell<RegisteredArena>}
unsafe impl Sync for UiArenaRegistry{}
/// 🗝️ Exclusive access to the registered pointer; a panic while it is held poisons the registry until its owner recovers it.
struct RegistryGuard<'a>{registry:&'a UiArenaRegistry,panicking:bool,_not_send:PhantomData<*mut ()>}
impl Deref for RegistryGuard<'_>{type Target=RegisteredArena;fn deref(&self)->&RegisteredArena{unsafe{&*self.registry.state.get()}}}
impl DerefMut for RegistryGuard<'_>{fn deref_mut(&mut self)->&mut RegisteredArena{unsafe{&mut*self.registry.state.get()}}}
impl Drop for RegistryGuard<'_>{fn drop(&mut self){if !self.panicking&&std::thread::panicking(){self.registry.poisoned.store(true,Ordering::Relaxed)}self.registry.held.store(false,Ordering::Release)}}
impl UiArenaRegistry{
 pub const fn new()->Self{Self{held:AtomicBool::new(false),poisoned:AtomicBool::new(false),state:UnsafeCell::new(RegisteredArena{pointer:0,epoch:0,closing:false})}}
 fn try_lock(&self)->Option<RegistryGuard<'_>>{self.held.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).ok()?;Some(RegistryGuard{registry:self,panicking:std::thread::panicking(),_not_send:PhantomData})}
 fn lock(&self)->RegistryGuard<'_>{loop{if let Some(guard)=self.try_lock(){return guard}std::hint::spin_loop();std::thread::yield_now()}}
 fn is_poisoned(&self)->bool{self.poisoned.load(Ordering::Relaxed)}
 fn clear_poison(&self){self.poisoned.store(false,Ordering::Relaxed)}
 pub fn borrow(&'static self)->Result<Option<UiArenaBorrow>,ValueError>{let Some(guard)=self.try_lock()else{return Ok(None)};if self.is_poisoned(){return Err(invalid())}if guard.pointer==0||guard.closing{""")
r("pub fn borrow_blocking(&'static self)->Result<UiArenaBorrow,ValueError>{let guard=self.state.lock().map_err(|_|invalid())?;if guard.pointer",
  "pub fn borrow_blocking(&'static self)->Result<UiArenaBorrow,ValueError>{let guard=self.lock();if self.is_poisoned(){return Err(invalid())}if guard.pointer")
r("pub struct UiArenaBorrow{guard:MutexGuard<'static,RegisteredArena>}","pub struct UiArenaBorrow{guard:RegistryGuard<'static>}")
r("let mut guard=match registry.state.try_lock(){Ok(guard)=>guard,Err(TryLockError::WouldBlock)=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(TryLockError::Poisoned(_))=>return Err(invalid())};",
  "let Some(mut guard)=registry.try_lock()else{return Ok(RetainedCloneStep::Progress(Default::default()))};if registry.is_poisoned(){return Err(invalid())}")
old="let mut guard=match self.registry.state.try_lock(){Ok(guard)=>guard,Err(TryLockError::WouldBlock)=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(TryLockError::Poisoned(error))=>error.into_inner()};"
assert s.count(old)==2;s=s.replace(old,"let Some(mut guard)=self.registry.try_lock()else{return Ok(RetainedCloneStep::Progress(Default::default()))};")
assert s.count("self.registry.state.clear_poison();")==2;s=s.replace("self.registry.state.clear_poison();","self.registry.clear_poison();")
assert "Mutex" not in s and "TryLockError" not in s and ".state.try_lock" not in s and ".state.clear" not in s
open(p,'w',encoding='utf-8').write(s)
