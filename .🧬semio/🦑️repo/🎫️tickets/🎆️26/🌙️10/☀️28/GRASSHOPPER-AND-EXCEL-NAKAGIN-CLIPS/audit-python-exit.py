"""🪟 Read recent native Python application-fault records without changing host state."""
import ctypes
from ctypes import wintypes as W
import json
from pathlib import Path
import sys
import xml.etree.ElementTree as ET
output=Path(__file__).parent/'🗑️generated'/'python-exit-events.json'
records=[]
if sys.platform=='win32':
    api=ctypes.WinDLL('wevtapi',use_last_error=True)
    api.EvtQuery.argtypes=[W.HANDLE,W.LPCWSTR,W.LPCWSTR,W.DWORD]
    api.EvtQuery.restype=W.HANDLE
    api.EvtNext.argtypes=[W.HANDLE,W.DWORD,ctypes.POINTER(W.HANDLE),W.DWORD,W.DWORD,ctypes.POINTER(W.DWORD)]
    api.EvtRender.argtypes=[W.HANDLE,W.HANDLE,W.DWORD,W.DWORD,W.LPVOID,ctypes.POINTER(W.DWORD),ctypes.POINTER(W.DWORD)]
    api.EvtClose.argtypes=[W.HANDLE]
    query=api.EvtQuery(None,'Application','*[System[Provider[@Name="Application Error"] and TimeCreated[timediff(@SystemTime)<=3600000]]]',0x201)
    if not query:
        raise ctypes.WinError(ctypes.get_last_error())
    try:
        handles=(W.HANDLE*64)()
        count=W.DWORD()
        if api.EvtNext(query,64,handles,0,0,ctypes.byref(count)):
            for handle in list(handles)[:count.value]:
                try:
                    needed,properties=W.DWORD(),W.DWORD()
                    api.EvtRender(None,handle,1,0,None,ctypes.byref(needed),ctypes.byref(properties))
                    buffer=ctypes.create_unicode_buffer(needed.value//ctypes.sizeof(ctypes.c_wchar)+1)
                    if not api.EvtRender(None,handle,1,ctypes.sizeof(buffer),buffer,ctypes.byref(needed),ctypes.byref(properties)):
                        raise ctypes.WinError(ctypes.get_last_error())
                    event=ET.fromstring(buffer.value)
                    data={row.attrib.get('Name',str(i)):row.text for i,row in enumerate(event.findall('.//{*}EventData/{*}Data'))}
                    if any('python' in str(v).lower() for v in data.values()):
                        created=event.find('.//{*}TimeCreated')
                        records.append({'time':created.attrib if created is not None else {},'data':data})
                finally:
                    api.EvtClose(handle)
    finally:
        api.EvtClose(query)
output.parent.mkdir(parents=True,exist_ok=True)
output.write_text(json.dumps({'platform':sys.platform,'records':records},indent=2),encoding='utf-8')
print('[DEBUG] Recent Python native fault records:',len(records))
