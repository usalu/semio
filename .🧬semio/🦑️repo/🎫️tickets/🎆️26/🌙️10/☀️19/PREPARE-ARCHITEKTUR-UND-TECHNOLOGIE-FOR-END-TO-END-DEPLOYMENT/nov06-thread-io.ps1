param([int]$WriterId, [string]$ExpectedBirth, [string]$ExpectedImage)
$ErrorActionPreference = 'Stop'
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class ThreadIo {
 [DllImport("kernel32.dll", SetLastError=true)] public static extern IntPtr OpenThread(uint access, bool inherit, uint id);
 [DllImport("kernel32.dll", SetLastError=true)] public static extern uint GetProcessIdOfThread(IntPtr thread);
 [DllImport("kernel32.dll", SetLastError=true)] [return: MarshalAs(UnmanagedType.Bool)] public static extern bool GetThreadIOPendingFlag(IntPtr thread, [MarshalAs(UnmanagedType.Bool)] out bool pending);
 [DllImport("kernel32.dll")] [return: MarshalAs(UnmanagedType.Bool)] public static extern bool CloseHandle(IntPtr handle);
}
'@
function Snapshot {
 $process = Get-Process -Id $WriterId -ErrorAction Stop
 $birth = $process.StartTime.ToUniversalTime().ToString('o'); $image = $process.Path
 if ([Math]::Abs(([datetime]$birth).Ticks - ([datetime]$ExpectedBirth).Ticks) -ge 10 -or [IO.Path]::GetFullPath($image) -ine [IO.Path]::GetFullPath($ExpectedImage)) { throw 'Process identity mismatch' }
 $threads = @($process.Threads | ForEach-Object { @{ id = $_.Id; birth = $_.StartTime.ToUniversalTime().ToString('o'); state = $_.ThreadState.ToString() } } | Sort-Object { $_.birth })
 if (-not $threads.Count) { throw 'No threads remain' }
 return @{ id = $process.Id; birth = $birth; image = $image; oldest = $threads[0] }
}
$before = Snapshot
$handle = [ThreadIo]::OpenThread(0x40, $false, [uint32]$before.oldest.id)
if ($handle -eq [IntPtr]::Zero) { throw "OpenThread failed: $([Runtime.InteropServices.Marshal]::GetLastWin32Error())" }
try {
 $owner = [ThreadIo]::GetProcessIdOfThread($handle)
 if ($owner -ne $WriterId) { throw 'Thread owner changed' }
 $pending = $false; $ok = [ThreadIo]::GetThreadIOPendingFlag($handle, [ref]$pending)
 $code = if ($ok) { 0 } else { [Runtime.InteropServices.Marshal]::GetLastWin32Error() }
} finally { [void][ThreadIo]::CloseHandle($handle) }
$after = Snapshot
if ($before.birth -ne $after.birth -or $before.image -ine $after.image -or $before.oldest.id -ne $after.oldest.id -or $before.oldest.birth -ne $after.oldest.birth) { throw 'Process or thread identity changed' }
@{ before = $before; after = $after; pending = $pending; ok = $ok; error = $code; owner = $owner } | ConvertTo-Json -Depth 6
