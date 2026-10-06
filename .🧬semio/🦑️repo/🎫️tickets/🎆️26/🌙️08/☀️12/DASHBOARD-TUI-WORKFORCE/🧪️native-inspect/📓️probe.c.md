#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <dbghelp.h>
#pragma function(memset)
void* memset(void* d,int v,size_t n){volatile unsigned char* p=d;while(n--)*p++=(unsigned char)v;return d;}
static void text(const char* s){DWORD n=0,w;while(s[n])n++;WriteFile(GetStdHandle(STD_OUTPUT_HANDLE),s,n,&w,NULL);}
static void number(unsigned long long n,int radix){char b[32];DWORD count=0,w;do{b[count++]="0123456789abcdef"[n%radix];n/=radix;}while(n);for(DWORD i=0;i<count/2;i++){char t=b[i];b[i]=b[count-i-1];b[count-i-1]=t;}WriteFile(GetStdHandle(STD_OUTPUT_HANDLE),b,count,&w,NULL);}
static void inspect(PROCESS_INFORMATION* p){
  DWORD previous=SuspendThread(p->hThread);text("[DEBUG] suspend previous=");number(previous,10);text("\n");
  CONTEXT c={0};c.ContextFlags=CONTEXT_FULL;if(!GetThreadContext(p->hThread,&c)){text("[DEBUG] context error=");number(GetLastError(),10);text("\n");ResumeThread(p->hThread);return;}
  text("[DEBUG] rip=0x");number(c.Rip,16);text(" rsp=0x");number(c.Rsp,16);text("\n");
  MEMORY_BASIC_INFORMATION memory={0};VirtualQueryEx(p->hProcess,(void*)c.Rip,&memory,sizeof(memory));text("[DEBUG] instruction allocation=0x");number((unsigned long long)memory.AllocationBase,16);text(" offset=0x");number(c.Rip-(unsigned long long)memory.AllocationBase,16);text("\n");
  wchar_t name[4096]={0};K32GetModuleFileNameExW(p->hProcess,(HMODULE)memory.AllocationBase,name,4096);char output[8192];int size=WideCharToMultiByte(CP_UTF8,0,name,-1,output,8192,NULL,NULL);if(size>0){text("[DEBUG] instruction module=");text(output);text("\n");}
  wchar_t symbolPath[8192]={0};GetEnvironmentVariableW(L"SEMIO_NATIVE_PROBE_SYMBOLS",symbolPath,8192);
  if(SymInitializeW(p->hProcess,symbolPath,TRUE)){
    STACKFRAME64 frame={0};frame.AddrPC.Offset=c.Rip;frame.AddrPC.Mode=AddrModeFlat;frame.AddrFrame.Offset=c.Rbp;frame.AddrFrame.Mode=AddrModeFlat;frame.AddrStack.Offset=c.Rsp;frame.AddrStack.Mode=AddrModeFlat;
    for(DWORD i=0;i<24;i++){
      if(!StackWalk64(IMAGE_FILE_MACHINE_AMD64,p->hProcess,p->hThread,&frame,&c,NULL,SymFunctionTableAccess64,SymGetModuleBase64,NULL))break;
      text("[DEBUG] stack ");number(i,10);text(" address=0x");number(frame.AddrPC.Offset,16);
      unsigned char storage[sizeof(SYMBOL_INFO)+2048]={0};SYMBOL_INFO* symbol=(SYMBOL_INFO*)storage;symbol->SizeOfStruct=sizeof(SYMBOL_INFO);symbol->MaxNameLen=2048;DWORD64 displacement=0;if(SymFromAddr(p->hProcess,frame.AddrPC.Offset,&displacement,symbol)){text(" ");text(symbol->Name);text("+0x");number(displacement,16);}text("\n");
    }
    SymCleanup(p->hProcess);
  }
  ResumeThread(p->hThread);
}
void ProbeMain(void){
  wchar_t* command=GetCommandLineW();while(*command==L' ')command++;if(*command==L'"'){command++;while(*command&&*command!=L'"')command++;if(*command)command++;}else while(*command&&*command!=L' ')command++;while(*command==L' ')command++;
  STARTUPINFOW startup={0};PROCESS_INFORMATION process={0};startup.cb=sizeof(startup);startup.dwFlags=STARTF_USESTDHANDLES;startup.hStdInput=GetStdHandle(STD_INPUT_HANDLE);startup.hStdOutput=GetStdHandle(STD_OUTPUT_HANDLE);startup.hStdError=GetStdHandle(STD_ERROR_HANDLE);
  if(!CreateProcessW(NULL,command,NULL,NULL,TRUE,CREATE_NO_WINDOW,NULL,NULL,&startup,&process)){text("[DEBUG] spawn error=");number(GetLastError(),10);text("\n");ExitProcess(1);}
  text("[DEBUG] native inspect pid=");number(process.dwProcessId,10);text(" thread=");number(process.dwThreadId,10);text("\n");
  if(WaitForSingleObject(process.hProcess,1000)==WAIT_TIMEOUT)inspect(&process);
  if(WaitForSingleObject(process.hProcess,20000)==WAIT_TIMEOUT){inspect(&process);text("[DEBUG] owned diagnostic deadline\n");TerminateProcess(process.hProcess,124);WaitForSingleObject(process.hProcess,5000);}
  DWORD status;GetExitCodeProcess(process.hProcess,&status);text("[DEBUG] native inspected exit=");number(status,10);text("\n");CloseHandle(process.hThread);CloseHandle(process.hProcess);ExitProcess(status);
}
