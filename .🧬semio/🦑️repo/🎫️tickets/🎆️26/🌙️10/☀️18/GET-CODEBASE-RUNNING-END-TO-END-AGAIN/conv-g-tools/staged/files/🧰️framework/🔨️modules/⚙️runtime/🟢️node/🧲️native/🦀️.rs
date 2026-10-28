#![no_std]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr::{null, null_mut};

type Env = *mut c_void;
type Value = *mut c_void;
type CallbackInfo = *mut c_void;
type Callback = unsafe extern "C" fn(Env, CallbackInfo) -> Value;
type Finalizer = unsafe extern "C" fn(Env, *mut c_void, *mut c_void);

const WORDS: usize = 8;
const NAME_BYTES: usize = 1024;

#[cfg_attr(windows, link(name = "node.exe", kind = "raw-dylib"))]
extern "C" {
    fn napi_create_function(env: Env, name: *const c_char, length: usize, callback: Callback, data: *mut c_void, result: *mut Value) -> c_int;
    fn napi_set_named_property(env: Env, object: Value, name: *const c_char, value: Value) -> c_int;
    fn napi_get_cb_info(env: Env, info: CallbackInfo, count: *mut usize, values: *mut Value, this_value: *mut Value, data: *mut *mut c_void) -> c_int;
    fn napi_get_value_bigint_uint64(env: Env, value: Value, result: *mut u64, lossless: *mut bool) -> c_int;
    fn napi_create_bigint_uint64(env: Env, value: u64, result: *mut Value) -> c_int;
    fn napi_get_value_string_utf8(env: Env, value: Value, buffer: *mut c_char, size: usize, result: *mut usize) -> c_int;
    #[cfg(windows)]
    fn napi_get_value_string_utf16(env: Env, value: Value, buffer: *mut u16, size: usize, result: *mut usize) -> c_int;
    fn napi_throw_error(env: Env, code: *const c_char, message: *const c_char) -> c_int;
    fn napi_is_typedarray(env: Env, value: Value, result: *mut bool) -> c_int;
    fn napi_get_typedarray_info(env: Env, value: Value, kind: *mut c_int, length: *mut usize, data: *mut *mut c_void, buffer: *mut Value, offset: *mut usize) -> c_int;
    fn napi_is_dataview(env: Env, value: Value, result: *mut bool) -> c_int;
    fn napi_get_dataview_info(env: Env, value: Value, length: *mut usize, data: *mut *mut c_void, buffer: *mut Value, offset: *mut usize) -> c_int;
    fn napi_create_arraybuffer(env: Env, length: usize, data: *mut *mut c_void, result: *mut Value) -> c_int;
    fn napi_create_external_arraybuffer(env: Env, data: *mut c_void, length: usize, finalizer: Option<Finalizer>, hint: *mut c_void, result: *mut Value) -> c_int;
}

#[cfg(unix)]
#[cfg_attr(target_os = "macos", link(name = "System"))]
#[cfg_attr(target_os = "linux", link(name = "c"))]
#[cfg_attr(target_os = "linux", link(name = "dl"))]
extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *const c_char;
    fn abort() -> !;
}

#[cfg(windows)]
#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    fn LoadLibraryW(path: *const u16) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
    fn FreeLibrary(module: *mut c_void) -> c_int;
    fn GetLastError() -> u32;
    fn ExitProcess(code: u32) -> !;
}

/// 💥️ A panic cannot unwind through the host: the process stops at once.
#[panic_handler]
fn halt(_: &core::panic::PanicInfo) -> ! {
    unsafe {
        #[cfg(unix)]
        abort();
        #[cfg(windows)]
        ExitProcess(70);
    }
}

/// 🚨️ Throws a JavaScript error with a NUL-terminated message and yields no value.
unsafe fn fail(env: Env, message: &'static [u8]) -> Value {
    napi_throw_error(env, null(), message.as_ptr() as *const c_char);
    null_mut()
}

/// 🔢️ Reads the call arguments into a fixed array and returns how many the caller passed; the others are null.
unsafe fn arguments<const N: usize>(env: Env, info: CallbackInfo) -> (usize, [Value; N]) {
    let mut count = N;
    let mut values = [null_mut(); N];
    napi_get_cb_info(env, info, &mut count, values.as_mut_ptr(), null_mut(), null_mut());
    for slot in values.iter_mut().skip(count) {
        *slot = null_mut();
    }
    (count, values)
}

/// 🔣️ Reads one exact unsigned 64-bit BigInt.
unsafe fn word(env: Env, value: Value) -> Option<u64> {
    let mut result = 0u64;
    let mut lossless = false;
    if value.is_null() || napi_get_value_bigint_uint64(env, value, &mut result, &mut lossless) != 0 || !lossless {
        return None;
    }
    Some(result)
}

/// 🎁️ Wraps one unsigned 64-bit word as a BigInt.
unsafe fn bigint(env: Env, value: u64) -> Value {
    let mut result = null_mut();
    napi_create_bigint_uint64(env, value, &mut result);
    result
}

/// 📚️ Writes a decimal number into a fixed buffer behind a prefix and returns the used length.
#[cfg(windows)]
fn decimal(buffer: &mut [u8], start: usize, mut value: u32) -> usize {
    let mut digits = [0u8; 10];
    let mut count = 0;
    loop {
        digits[count] = b'0' + (value % 10) as u8;
        count += 1;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    let mut at = start;
    while count > 0 && at < buffer.len() - 1 {
        count -= 1;
        buffer[at] = digits[count];
        at += 1;
    }
    buffer[at] = 0;
    at
}

/// 🔓️ Opens one dynamic library by name or path and returns its handle.
unsafe extern "C" fn open(env: Env, info: CallbackInfo) -> Value {
    let (_, [path]) = arguments::<1>(env, info);
    #[cfg(unix)]
    {
        let mut name = [0 as c_char; NAME_BYTES];
        let mut length = 0usize;
        if napi_get_value_string_utf8(env, path, name.as_mut_ptr(), NAME_BYTES, &mut length) != 0 || length + 1 >= NAME_BYTES {
            return fail(env, b"Library name of at most 1023 UTF-8 bytes required\0");
        }
        let handle = dlopen(name.as_ptr(), 2);
        if handle.is_null() {
            let reason = dlerror();
            return if reason.is_null() { fail(env, b"dlopen failed\0") } else {
                napi_throw_error(env, null(), reason);
                null_mut()
            };
        }
        bigint(env, handle as usize as u64)
    }
    #[cfg(windows)]
    {
        let mut name = [0u16; NAME_BYTES];
        let mut length = 0usize;
        if napi_get_value_string_utf16(env, path, name.as_mut_ptr(), NAME_BYTES, &mut length) != 0 || length + 1 >= NAME_BYTES {
            return fail(env, b"Library name of at most 1023 UTF-16 units required\0");
        }
        let handle = LoadLibraryW(name.as_ptr());
        if handle.is_null() {
            let mut message = *b"LoadLibraryW failed with code \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
            decimal(&mut message, 30, GetLastError());
            napi_throw_error(env, null(), message.as_ptr() as *const c_char);
            return null_mut();
        }
        bigint(env, handle as usize as u64)
    }
}

/// 🔎️ Resolves one exported symbol of an opened library to its address.
unsafe extern "C" fn symbol(env: Env, info: CallbackInfo) -> Value {
    let (_, [handle, name]) = arguments::<2>(env, info);
    let Some(handle) = word(env, handle) else { return fail(env, b"Library handle required\0") };
    let mut text = [0 as c_char; NAME_BYTES];
    let mut length = 0usize;
    if napi_get_value_string_utf8(env, name, text.as_mut_ptr(), NAME_BYTES, &mut length) != 0 || length + 1 >= NAME_BYTES {
        return fail(env, b"Symbol name of at most 1023 UTF-8 bytes required\0");
    }
    #[cfg(unix)]
    let address = dlsym(handle as usize as *mut c_void, text.as_ptr());
    #[cfg(windows)]
    let address = GetProcAddress(handle as usize as *mut c_void, text.as_ptr());
    if address.is_null() {
        return fail(env, b"Symbol not found in library\0");
    }
    bigint(env, address as usize as u64)
}

/// 🔒️ Releases one opened library.
unsafe extern "C" fn close(env: Env, info: CallbackInfo) -> Value {
    let (_, [handle]) = arguments::<1>(env, info);
    let Some(handle) = word(env, handle) else { return fail(env, b"Library handle required\0") };
    #[cfg(unix)]
    let status = dlclose(handle as usize as *mut c_void);
    #[cfg(windows)]
    let status = if FreeLibrary(handle as usize as *mut c_void) != 0 { 0 } else { 1 };
    bigint(env, status as u32 as u64)
}

/// ☎️ Calls one C function with up to eight integer-class words and returns the full return register.
unsafe extern "C" fn call(env: Env, info: CallbackInfo) -> Value {
    let (count, values) = arguments::<{ WORDS + 1 }>(env, info);
    let Some(address) = word(env, values[0]) else { return fail(env, b"Function address required\0") };
    if address == 0 {
        return fail(env, b"Function address required\0");
    }
    let mut words = [0u64; WORDS];
    for index in 0..WORDS {
        let value = values[index + 1];
        if index + 1 >= count || value.is_null() {
            continue;
        }
        let Some(read) = word(env, value) else { return fail(env, b"Argument words must be exact unsigned 64-bit BigInts\0") };
        words[index] = read;
    }
    let function: unsafe extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 = core::mem::transmute(address as usize);
    let result = function(words[0], words[1], words[2], words[3], words[4], words[5], words[6], words[7]);
    bigint(env, result)
}

/// 📍️ Gives the machine address of the first byte of a typed array, Buffer, DataView or ArrayBuffer view.
unsafe extern "C" fn address_of(env: Env, info: CallbackInfo) -> Value {
    let (_, [view]) = arguments::<1>(env, info);
    let mut typed = false;
    let mut data: *mut c_void = null_mut();
    let mut length = 0usize;
    let mut buffer = null_mut();
    let mut offset = 0usize;
    let mut kind: c_int = 0;
    if view.is_null() || napi_is_typedarray(env, view, &mut typed) != 0 {
        return fail(env, b"Typed array or DataView required\0");
    }
    let status = if typed {
        napi_get_typedarray_info(env, view, &mut kind, &mut length, &mut data, &mut buffer, &mut offset)
    } else {
        let mut data_view = false;
        if napi_is_dataview(env, view, &mut data_view) != 0 || !data_view {
            return fail(env, b"Typed array or DataView required\0");
        }
        napi_get_dataview_info(env, view, &mut length, &mut data, &mut buffer, &mut offset)
    };
    if status != 0 {
        return fail(env, b"Typed array or DataView required\0");
    }
    bigint(env, data as usize as u64)
}

/// 📖️ Copies a range of machine memory into a fresh ArrayBuffer.
unsafe extern "C" fn peek(env: Env, info: CallbackInfo) -> Value {
    let (_, [address, length]) = arguments::<2>(env, info);
    let (Some(address), Some(length)) = (word(env, address), word(env, length)) else { return fail(env, b"Address and length required\0") };
    if address == 0 {
        return fail(env, b"Address required\0");
    }
    let mut data: *mut c_void = null_mut();
    let mut result = null_mut();
    if napi_create_arraybuffer(env, length as usize, &mut data, &mut result) != 0 {
        return fail(env, b"ArrayBuffer allocation failed\0");
    }
    if length != 0 {
        core::ptr::copy_nonoverlapping(address as usize as *const u8, data as *mut u8, length as usize);
    }
    result
}

/// 🪟️ Exposes a range of machine memory as an ArrayBuffer without copying; the memory stays owned by its provider.
unsafe extern "C" fn view(env: Env, info: CallbackInfo) -> Value {
    let (_, [address, length]) = arguments::<2>(env, info);
    let (Some(address), Some(length)) = (word(env, address), word(env, length)) else { return fail(env, b"Address and length required\0") };
    if address == 0 {
        return fail(env, b"Address required\0");
    }
    let mut result = null_mut();
    if napi_create_external_arraybuffer(env, address as usize as *mut c_void, length as usize, None, null_mut(), &mut result) != 0 {
        return fail(env, b"External ArrayBuffer unsupported by this host\0");
    }
    result
}

/// 🧩️ Adds one named function to the exports object.
unsafe fn export(env: Env, exports: Value, name: &'static [u8], callback: Callback) {
    let mut function = null_mut();
    napi_create_function(env, name.as_ptr() as *const c_char, name.len() - 1, callback, null_mut(), &mut function);
    napi_set_named_property(env, exports, name.as_ptr() as *const c_char, function);
}

/// 🔌️ Registers the seven primitives of the Node native core.
#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(env: Env, exports: Value) -> Value {
    export(env, exports, b"open\0", open);
    export(env, exports, b"symbol\0", symbol);
    export(env, exports, b"close\0", close);
    export(env, exports, b"call\0", call);
    export(env, exports, b"addressOf\0", address_of);
    export(env, exports, b"peek\0", peek);
    export(env, exports, b"view\0", view);
    exports
}

/// 🏷️ Declares the Node-API version this addon targets.
#[no_mangle]
pub extern "C" fn node_api_module_get_api_version_v1() -> i32 {
    8
}
