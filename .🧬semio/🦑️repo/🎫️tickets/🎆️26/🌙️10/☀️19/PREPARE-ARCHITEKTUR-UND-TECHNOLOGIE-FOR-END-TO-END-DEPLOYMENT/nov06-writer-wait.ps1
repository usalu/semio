param([int]$WriterId, [string]$ExpectedBirth, [string]$ExpectedImage)
$ErrorActionPreference = 'Stop'
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class WriterWait {
 [StructLayout(LayoutKind.Explicit, Size=280, CharSet=CharSet.Unicode)] public struct Node {
  [FieldOffset(0)] public int Type;
  [FieldOffset(4)] public int Status;
  [FieldOffset(8)] public uint ProcessId;
  [FieldOffset(12)] public uint ThreadId;
  [FieldOffset(16)] public uint WaitTime;
  [FieldOffset(20)] public uint ContextSwitches;
 }
 [DllImport("advapi32.dll", SetLastError=true)] public static extern IntPtr OpenThreadWaitChainSession(uint flags, IntPtr callback);
 [DllImport("advapi32.dll", SetLastError=true)] [return: MarshalAs(UnmanagedType.Bool)] public static extern bool GetThreadWaitChain(IntPtr session, UIntPtr context, uint flags, uint threadId, ref uint count, [Out] Node[] nodes, [MarshalAs(UnmanagedType.Bool)] out bool cycle);
 [DllImport("advapi32.dll")] public static extern void CloseThreadWaitChainSession(IntPtr session);
}
'@
function Identity {
 $item = Get-Process -Id $WriterId -ErrorAction Stop
 $birth = $item.StartTime.ToUniversalTime().ToString('o')
 $image = $item.Path
 if ([Math]::Abs(([datetime]$birth).Ticks - ([datetime]$ExpectedBirth).Ticks) -ge 10 -or [IO.Path]::GetFullPath($image) -ine [IO.Path]::GetFullPath($ExpectedImage)) { throw "Writer process identity mismatch: $birth $image" }
 return @{ id = $item.Id; birth = $birth; image = $image }
}
$before = Identity
$threads = @(Get-Process -Id $WriterId | ForEach-Object { $_.Threads } | ForEach-Object {
 $state = $_.ThreadState.ToString()
 $reason = if ($state -eq 'Wait') { $_.WaitReason.ToString() } else { $null }
 @{ id = $_.Id; birth = $_.StartTime.ToUniversalTime().ToString('o'); state = $state; reason = $reason }
} | Sort-Object { $_.birth })
if (-not $threads.Count) { throw 'No writer threads remain' }
$session = [WriterWait]::OpenThreadWaitChainSession(0, [IntPtr]::Zero)
if ($session -eq [IntPtr]::Zero) { throw "OpenThreadWaitChainSession failed: $([Runtime.InteropServices.Marshal]::GetLastWin32Error())" }
try {
 $nodes = [WriterWait+Node[]]::new(16); [uint32]$count = 16; $cycle = $false
 $ok = [WriterWait]::GetThreadWaitChain($session, [UIntPtr]::Zero, 0, [uint32]$threads[0].id, [ref]$count, $nodes, [ref]$cycle)
 $errorCode = if ($ok) { 0 } else { [Runtime.InteropServices.Marshal]::GetLastWin32Error() }
 $chain = @(); for ($i = 0; $i -lt [Math]::Min($count,16); $i++) { $node = $nodes[$i]; $chain += @{ type = $node.Type; status = $node.Status; process = if ($node.Type -eq 8) { $node.ProcessId } else { $null }; thread = if ($node.Type -eq 8) { $node.ThreadId } else { $null }; waitMs = if ($node.Type -eq 8) { $node.WaitTime } else { $null } } }
} finally { [WriterWait]::CloseThreadWaitChainSession($session) }
$after = Identity
if ($before.birth -ne $after.birth -or $before.image -ine $after.image) { throw 'Writer process identity changed during query' }
@{ before = $before; after = $after; threads = $threads; oldestThread = $threads[0].id; waitChain = @{ ok = $ok; error = $errorCode; count = $count; cycle = $cycle; nodes = $chain; flags = 0 } } | ConvertTo-Json -Depth 8
