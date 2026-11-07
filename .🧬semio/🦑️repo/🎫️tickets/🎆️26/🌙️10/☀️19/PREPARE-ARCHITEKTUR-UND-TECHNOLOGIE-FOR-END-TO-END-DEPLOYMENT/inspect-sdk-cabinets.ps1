param([Parameter(Mandatory = $true)][string]$Directory)
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$installer = New-Object -ComObject WindowsInstaller.Installer
$result = @()
for ($i = 0; $i -lt 8; $i++) {
    $path = Join-Path $Directory "sdk-$i.msi"
    $db = $installer.OpenDatabase($path, 0)
    $view = $db.OpenView("SELECT ``Cabinet`` FROM ``Media``")
    $view.Execute()
    while ($record = $view.Fetch()) {
        $value = $record.StringData(1)
        if ($value) { $result += [pscustomobject]@{ index = $i; cabinet = $value } }
    }
    $view.Close()
}
ConvertTo-Json -InputObject @($result) -Compress
