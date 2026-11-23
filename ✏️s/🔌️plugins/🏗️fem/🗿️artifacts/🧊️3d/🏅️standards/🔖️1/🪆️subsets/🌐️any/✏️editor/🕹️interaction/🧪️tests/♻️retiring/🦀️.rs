//! ♻️ Test-kit guard: an original value held outside a store retires through a funded original fixture when the law ends.

/// ♻️ Retires one original value, because the sealed storage refuses a plain drop of its last owner.
pub(crate) fn retire<T: semio_framework_value::retirement::RetireOwned>(value: T) {
    let mut fixture = semio_plugin::artifact_app_laws::OriginalWorkFixture::new();
    fixture.retire_owned(value, semio_plugin::artifact_app_laws::OriginalWorkFixture::stock());
    fixture.finish();
}

/// 🧷️ An original value that retires itself when the test scope ends.
pub(crate) struct Retiring<T: semio_framework_value::retirement::RetireOwned>(Option<T>);

impl<T: semio_framework_value::retirement::RetireOwned> Retiring<T> {
    pub(crate) fn new(value: T) -> Self { Self(Some(value)) }
}

impl<T: semio_framework_value::retirement::RetireOwned> std::ops::Deref for Retiring<T> {
    type Target = T;
    fn deref(&self) -> &T { self.0.as_ref().expect("retiring value is live") }
}

impl<T: semio_framework_value::retirement::RetireOwned> std::ops::DerefMut for Retiring<T> {
    fn deref_mut(&mut self) -> &mut T { self.0.as_mut().expect("retiring value is live") }
}

impl<T: semio_framework_value::retirement::RetireOwned> Drop for Retiring<T> {
    fn drop(&mut self) {
        if let Some(value) = self.0.take() {
            if std::thread::panicking() { std::mem::forget(value); } else { retire(value); }
        }
    }
}
