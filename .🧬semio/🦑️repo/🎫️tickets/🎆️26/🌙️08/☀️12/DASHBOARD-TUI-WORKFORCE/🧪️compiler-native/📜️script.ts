import {mkdirSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {spawnSync} from "node:child_process";

const output=resolve(import.meta.dir,"../🗑️generated/compiler-probe");mkdirSync(output,{recursive:true});
const compiler=process.argv[2]!, sdk=process.argv[3]!, sdkVersion=process.argv[4]!, source=join(output,"📜️native-script.ts"), binary=join(output,"native-compiler-probe.exe");
writeFileSync(source,String.raw`#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#pragma function(memset)
void* memset(void* destination,int value,size_t count){volatile unsigned char* p=destination;while(count--)*p++=(unsigned char)value;return destination;}
static DWORD length(const wchar_t* value){DWORD n=0;while(value[n])n++;return n;}
static wchar_t fold(wchar_t c){if(c==L'/')return L'\\';return c>=L'A'&&c<=L'Z'?c+32:c;}
static int inside(const wchar_t* path,const wchar_t* root){DWORD n=0;while(root[n]){if(!path[n]||fold(path[n])!=fold(root[n]))return 0;n++;}return path[n]==L'\\'||path[n]==L'/';}
static wchar_t* variable(const wchar_t* name){DWORD n=GetEnvironmentVariableW(name,NULL,0);wchar_t* p=HeapAlloc(GetProcessHeap(),HEAP_ZERO_MEMORY,(n+1)*sizeof(wchar_t));if(n)GetEnvironmentVariableW(name,p,n);return p;}
static int metadata_only(const wchar_t* path,const wchar_t* root){
  if(!root[0]||!inside(path,root))return 0;
  DWORD n=length(path);wchar_t* pattern=HeapAlloc(GetProcessHeap(),0,(n+7)*sizeof(wchar_t));
  for(DWORD i=0;i<n;i++)pattern[i]=path[i];const wchar_t* suffix=L"\\*.dll";for(DWORD i=0;i<7;i++)pattern[n+i]=suffix[i];
  WIN32_FIND_DATAW data;HANDLE file=FindFirstFileW(pattern,&data);DWORD error=GetLastError();HeapFree(GetProcessHeap(),0,pattern);if(file!=INVALID_HANDLE_VALUE){FindClose(file);return 0;}return error==ERROR_FILE_NOT_FOUND;
}
static void number(DWORD n){char b[32];DWORD count=0,done;do{b[count++]=(char)('0'+n%10);n/=10;}while(n);for(DWORD i=0;i<count/2;i++){char t=b[i];b[i]=b[count-i-1];b[count-i-1]=t;}WriteFile(GetStdHandle(STD_ERROR_HANDLE),b,count,&done,NULL);}
static void text(const char* s){DWORD n=0,done;while(s[n])n++;WriteFile(GetStdHandle(STD_ERROR_HANDLE),s,n,&done,NULL);}
void CompilerProbeMain(void){
  wchar_t* original=variable(L"PATH"),* root=variable(L"CARGO_BUILD_BUILD_DIR"),* enabled=variable(L"SEMIO_TEST_DLL_PATH_FILTER");DWORD size=length(original),written=0,omitted=0;
  wchar_t* output=HeapAlloc(GetProcessHeap(),HEAP_ZERO_MEMORY,(size+1)*sizeof(wchar_t));
  for(DWORD start=0;start<=size;){DWORD end=start;while(end<size&&original[end]!=L';')end++;wchar_t separator=original[end];original[end]=0;
    if(enabled[0]==L'1'&&metadata_only(original+start,root))omitted++;
    else{if(written)output[written++]=L';';for(DWORD i=start;i<end;i++)output[written++]=original[i];}
    original[end]=separator;start=end+1;
  }
  text("[DEBUG] compiler path chars=");number(size);text(" omitted=");number(omitted);text(" finalChars=");number(written);text("\n");
  SetEnvironmentVariableW(L"PATH",output);
  wchar_t* command=GetCommandLineW();while(*command==L' ')command++;if(*command==L'"'){command++;while(*command&&*command!=L'"')command++;if(*command)command++;}else while(*command&&*command!=L' ')command++;while(*command==L' ')command++;
  STARTUPINFOW startup={0};PROCESS_INFORMATION process={0};startup.cb=sizeof(startup);startup.dwFlags=STARTF_USESTDHANDLES;startup.hStdInput=GetStdHandle(STD_INPUT_HANDLE);startup.hStdOutput=GetStdHandle(STD_OUTPUT_HANDLE);startup.hStdError=GetStdHandle(STD_ERROR_HANDLE);
  if(!CreateProcessW(NULL,command,NULL,NULL,TRUE,0,NULL,NULL,&startup,&process)){text("[DEBUG] compiler spawn error=");number(GetLastError());text("\n");ExitProcess(1);}
  CloseHandle(process.hThread);WaitForSingleObject(process.hProcess,INFINITE);DWORD status;GetExitCodeProcess(process.hProcess,&status);CloseHandle(process.hProcess);ExitProcess(status);
}`);
const args=["/nologo","/O2","/Oi","/GS-","/Zl","/Gs99999999","/Tc"+source,"/Fo"+join(output,"native.obj"),"/Fe"+binary,"/I"+join(dirname(compiler),"../../../include"),"/I"+join(sdk,"Include",sdkVersion,"shared"),"/I"+join(sdk,"Include",sdkVersion,"um"),"/I"+join(sdk,"Include",sdkVersion,"ucrt"),"/link","/NODEFAULTLIB","/ENTRY:CompilerProbeMain","/SUBSYSTEM:CONSOLE",join(sdk,"Lib",sdkVersion,"um/x64/kernel32.lib")];
const result=spawnSync(compiler,args,{stdio:"inherit",windowsHide:true});if(result.status!==0)throw Error("Native compiler probe build failed");console.log(binary);
