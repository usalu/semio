<#
.SYNOPSIS
🧭 Lists processes whose current directory starts with the given prefix (reads each PEB): process-cwd.ps1 -Prefix C:\git\semio
#>
param([Parameter(Mandatory)][string]$Prefix)
Add-Type @"
using System;using System.Runtime.InteropServices;using System.Text;
public static class Cwd{
 [DllImport("kernel32.dll")]static extern IntPtr OpenProcess(int a,bool i,int p);
 [DllImport("kernel32.dll")]static extern bool CloseHandle(IntPtr h);
 [DllImport("ntdll.dll")]static extern int NtQueryInformationProcess(IntPtr h,int c,ref PBI i,int l,out int r);
 [DllImport("kernel32.dll")]static extern bool ReadProcessMemory(IntPtr h,IntPtr a,byte[] b,int s,out int r);
 [StructLayout(LayoutKind.Sequential)]struct PBI{public IntPtr a;public IntPtr Peb;public IntPtr c;public IntPtr d;public IntPtr e;public IntPtr f;}
 static IntPtr Ptr(IntPtr h,IntPtr a){byte[] b=new byte[8];int r;return ReadProcessMemory(h,a,b,8,out r)?(IntPtr)BitConverter.ToInt64(b,0):IntPtr.Zero;}
 public static string Of(int pid){
  IntPtr h=OpenProcess(0x0410,false,pid);if(h==IntPtr.Zero)return null;
  try{PBI i=new PBI();int r;if(NtQueryInformationProcess(h,0,ref i,Marshal.SizeOf(i),out r)!=0)return null;
   IntPtr pp=Ptr(h,i.Peb+0x20);if(pp==IntPtr.Zero)return null;
   byte[] u=new byte[16];if(!ReadProcessMemory(h,pp+0x38,u,16,out r))return null;
   int len=BitConverter.ToUInt16(u,0);IntPtr buf=(IntPtr)BitConverter.ToInt64(u,8);byte[] s=new byte[len];
   return ReadProcessMemory(h,buf,s,len,out r)?Encoding.Unicode.GetString(s):null;
  }finally{CloseHandle(h);}
 }
}
"@
Get-CimInstance Win32_Process | ForEach-Object {
 $cwd = [Cwd]::Of($_.ProcessId)
 if ($cwd -and $cwd.TrimEnd('\').StartsWith($Prefix, [StringComparison]::OrdinalIgnoreCase) -and $cwd.TrimEnd('\').Length -eq $Prefix.Length) {
  [pscustomobject]@{ Pid = $_.ProcessId; Parent = $_.ParentProcessId; Name = $_.Name; Started = $_.CreationDate; Cwd = $cwd; Cmd = if ($_.CommandLine) { $_.CommandLine.Substring(0, [Math]::Min(200, $_.CommandLine.Length)) } }
 }
} | Format-List
