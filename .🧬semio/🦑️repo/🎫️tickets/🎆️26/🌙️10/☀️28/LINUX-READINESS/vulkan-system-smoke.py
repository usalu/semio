import ctypes
import json

class ApplicationInfo(ctypes.Structure):
    _fields_ = [("sType", ctypes.c_uint32), ("pNext", ctypes.c_void_p), ("pApplicationName", ctypes.c_char_p), ("applicationVersion", ctypes.c_uint32), ("pEngineName", ctypes.c_char_p), ("engineVersion", ctypes.c_uint32), ("apiVersion", ctypes.c_uint32)]

class InstanceCreateInfo(ctypes.Structure):
    _fields_ = [("sType", ctypes.c_uint32), ("pNext", ctypes.c_void_p), ("flags", ctypes.c_uint32), ("pApplicationInfo", ctypes.POINTER(ApplicationInfo)), ("enabledLayerCount", ctypes.c_uint32), ("ppEnabledLayerNames", ctypes.POINTER(ctypes.c_char_p)), ("enabledExtensionCount", ctypes.c_uint32), ("ppEnabledExtensionNames", ctypes.POINTER(ctypes.c_char_p))]

library = ctypes.CDLL("libvulkan.so.1")
library.vkCreateInstance.argtypes = [ctypes.POINTER(InstanceCreateInfo), ctypes.c_void_p, ctypes.POINTER(ctypes.c_void_p)]
library.vkCreateInstance.restype = ctypes.c_int32
library.vkEnumeratePhysicalDevices.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint32), ctypes.POINTER(ctypes.c_void_p)]
library.vkEnumeratePhysicalDevices.restype = ctypes.c_int32
library.vkGetPhysicalDeviceProperties.argtypes = [ctypes.c_void_p, ctypes.c_void_p]
library.vkDestroyInstance.argtypes = [ctypes.c_void_p, ctypes.c_void_p]
application = ApplicationInfo(sType=0, pApplicationName=b"semio-readiness", apiVersion=1 << 22)
create = InstanceCreateInfo(sType=1, pApplicationInfo=ctypes.pointer(application))
instance = ctypes.c_void_p()
status = library.vkCreateInstance(ctypes.byref(create), None, ctypes.byref(instance))
assert status == 0, status
try:
    count = ctypes.c_uint32()
    assert library.vkEnumeratePhysicalDevices(instance, ctypes.byref(count), None) == 0
    assert count.value > 0
    devices = (ctypes.c_void_p * count.value)()
    assert library.vkEnumeratePhysicalDevices(instance, ctypes.byref(count), devices) == 0
    rows = []
    for device in devices:
        properties = ctypes.create_string_buffer(4096)
        library.vkGetPhysicalDeviceProperties(device, properties)
        kind = ctypes.c_uint32.from_buffer(properties, 16).value
        rows.append({"name": properties.raw[20:276].split(b"\0", 1)[0].decode(), "type": kind})
    assert any(row["type"] == 4 for row in rows), rows
    print(json.dumps({"loader": "libvulkan.so.1", "instanceCreated": True, "devices": rows, "softwareIcd": True}))
finally:
    library.vkDestroyInstance(instance, None)
