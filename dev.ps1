$jobs = @()

try {
    $jobs += Start-Job -ScriptBlock {
        .\python\engine\dev.ps1
    }
    $jobs += Start-Job -ScriptBlock {
        .\javascript\dev.ps1
    }

    Get-Job | Wait-Job | Receive-Job
} finally {
    # Clean up jobs if the script is canceled
    foreach ($job in $jobs) {
        if ($job.State -eq 'Running') {
            Stop-Job -Job $job
        }
        Remove-Job -Job $job
    }
}