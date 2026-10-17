"""🧵 rw2t step C: the per-thread scene cell follows the testkit convention, so the companion bins and the crates' dev builds get per-thread scene state too."""
from ed import *
T_ = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
replace(T_ + "/🎭️scene-state/🦀️.rs", [
 ('pub struct SceneCell<T>(#[cfg(not(test))] OnceLock<Mutex<RefCell<T>>>, #[cfg(test)] std::marker::PhantomData<fn() -> T>);',
  'pub struct SceneCell<T>(#[cfg(not(any(test, feature = "testkit")))] OnceLock<Mutex<RefCell<T>>>, #[cfg(any(test, feature = "testkit"))] std::marker::PhantomData<fn() -> T>);', 1),
 ('        #[cfg(not(test))]\n        return Self(OnceLock::new());\n        #[cfg(test)]\n        return Self(std::marker::PhantomData);',
  '        #[cfg(not(any(test, feature = "testkit")))]\n        return Self(OnceLock::new());\n        #[cfg(any(test, feature = "testkit"))]\n        return Self(std::marker::PhantomData);', 1),
 ('        #[cfg(not(test))]\n        let state = self.0.get_or_init(|| Mutex::new(RefCell::new(T::default())));\n        #[cfg(test)]\n        let state = crate::element_contracts::test_worker_cell',
  '        #[cfg(not(any(test, feature = "testkit")))]\n        let state = self.0.get_or_init(|| Mutex::new(RefCell::new(T::default())));\n        #[cfg(any(test, feature = "testkit"))]\n        let state = crate::element_contracts::test_worker_cell', 1),
])
