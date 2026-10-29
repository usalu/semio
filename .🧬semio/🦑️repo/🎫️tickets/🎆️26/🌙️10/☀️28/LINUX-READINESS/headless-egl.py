import ctypes as c
import json
import os

egl=c.CDLL("libEGL.so.1")
gles=c.CDLL("libGLESv2.so.2")
def bind(library,name,args,result):
    fn=getattr(library,name)
    fn.argtypes=args
    fn.restype=result
    return fn
ptr=c.c_void_p
integer=c.c_int
get_display=bind(egl,"eglGetDisplay",[ptr],ptr)
initialize=bind(egl,"eglInitialize",[ptr,c.POINTER(integer),c.POINTER(integer)],integer)
choose=bind(egl,"eglChooseConfig",[ptr,c.POINTER(integer),c.POINTER(ptr),integer,c.POINTER(integer)],integer)
bind_api=bind(egl,"eglBindAPI",[c.c_uint],integer)
create_context=bind(egl,"eglCreateContext",[ptr,ptr,ptr,c.POINTER(integer)],ptr)
create_surface=bind(egl,"eglCreatePbufferSurface",[ptr,ptr,c.POINTER(integer)],ptr)
make_current=bind(egl,"eglMakeCurrent",[ptr,ptr,ptr,ptr],integer)
query=bind(egl,"eglQueryString",[ptr,integer],c.c_char_p)
gl_string=bind(gles,"glGetString",[c.c_uint],c.c_char_p)
destroy_context=bind(egl,"eglDestroyContext",[ptr,ptr],integer)
destroy_surface=bind(egl,"eglDestroySurface",[ptr,ptr],integer)
terminate=bind(egl,"eglTerminate",[ptr],integer)
display=get_display(None)
major,minor=integer(),integer()
context=surface=None
try:
    if not initialize(display,c.byref(major),c.byref(minor)):
        raise RuntimeError("eglInitialize failed")
    config,count=ptr(),integer()
    attributes=(integer*13)(0x3033,1,0x3040,4,0x3024,8,0x3023,8,0x3022,8,0x3025,0,0x3038)
    if not choose(display,attributes,c.byref(config),1,c.byref(count)) or count.value!=1:
        raise RuntimeError("EGL ES2 pbuffer configuration unavailable")
    if not bind_api(0x30A0):
        raise RuntimeError("eglBindAPI failed")
    context=create_context(display,config,None,(integer*3)(0x3098,2,0x3038))
    surface=create_surface(display,config,(integer*5)(0x3057,16,0x3056,16,0x3038))
    if not context or not surface or not make_current(display,surface,surface,context):
        raise RuntimeError("EGL ES2 context unavailable")
    decode=lambda value:value.decode() if value else None
    print(json.dumps({"DISPLAY":os.environ.get("DISPLAY"),"eglVersion":[major.value,minor.value],"eglVendor":decode(query(display,0x3053)),"eglApis":decode(query(display,0x308D)),"glVendor":decode(gl_string(0x1F00)),"glRenderer":decode(gl_string(0x1F01)),"glVersion":decode(gl_string(0x1F02))}))
finally:
    if display:
        make_current(display,None,None,None)
        if surface:
            destroy_surface(display,surface)
        if context:
            destroy_context(display,context)
        terminate(display)

x11=c.CDLL("libX11.so.6")
gl=c.CDLL("libGL.so.1")
open_display=bind(x11,"XOpenDisplay",[c.c_char_p],ptr)
default_screen=bind(x11,"XDefaultScreen",[ptr],integer)
close_display=bind(x11,"XCloseDisplay",[ptr],integer)
free=bind(x11,"XFree",[ptr],integer)
query_version=bind(gl,"glXQueryVersion",[ptr,c.POINTER(integer),c.POINTER(integer)],integer)
choose_fb=bind(gl,"glXChooseFBConfig",[ptr,integer,c.POINTER(integer),c.POINTER(integer)],c.POINTER(ptr))
create_glx_context=bind(gl,"glXCreateNewContext",[ptr,ptr,integer,ptr,integer],ptr)
create_pbuffer=bind(gl,"glXCreatePbuffer",[ptr,ptr,c.POINTER(integer)],c.c_ulong)
make_glx_current=bind(gl,"glXMakeContextCurrent",[ptr,c.c_ulong,c.c_ulong,ptr],integer)
destroy_pbuffer=bind(gl,"glXDestroyPbuffer",[ptr,c.c_ulong],None)
destroy_glx_context=bind(gl,"glXDestroyContext",[ptr,ptr],None)
desktop_string=bind(gl,"glGetString",[c.c_uint],c.c_char_p)
xdisplay=open_display(None)
configs=glx_context=pbuffer=None
try:
    if not xdisplay:
        raise RuntimeError("XOpenDisplay failed for GLX")
    glx_major,glx_minor,count=integer(),integer(),integer()
    if not query_version(xdisplay,c.byref(glx_major),c.byref(glx_minor)) or (glx_major.value,glx_minor.value)<(1,3):
        raise RuntimeError("GLX 1.3 unavailable")
    configs=choose_fb(xdisplay,default_screen(xdisplay),(integer*5)(0x8011,1,0x8010,4,0),c.byref(count))
    if not configs or count.value<1:
        raise RuntimeError("GLX pbuffer configuration unavailable")
    glx_context=create_glx_context(xdisplay,configs[0],0x8014,None,1)
    pbuffer=create_pbuffer(xdisplay,configs[0],(integer*5)(0x8041,16,0x8040,16,0))
    if not glx_context or not pbuffer or not make_glx_current(xdisplay,pbuffer,pbuffer,glx_context):
        raise RuntimeError("GLX desktop context unavailable")
    print(json.dumps({"glxVersion":[glx_major.value,glx_minor.value],"glVendor":desktop_string(0x1F00).decode(),"glRenderer":desktop_string(0x1F01).decode(),"glVersion":desktop_string(0x1F02).decode()}))
finally:
    if xdisplay:
        make_glx_current(xdisplay,0,0,None)
        if pbuffer:
            destroy_pbuffer(xdisplay,pbuffer)
        if glx_context:
            destroy_glx_context(xdisplay,glx_context)
        if configs:
            free(configs)
        close_display(xdisplay)
