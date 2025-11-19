param(
    [Parameter(Mandatory=$true)]
    [string]$Path
)

function Sort-JsonKeys {
    param($Object)
    
    if ($null -eq $Object) {
        return $null
    }
    
    if ($Object -is [System.Collections.IDictionary] -or $Object.GetType().Name -eq 'PSCustomObject') {
        # Convert PSCustomObject to hashtable for sorting
        $hash = @{}
        if ($Object -is [System.Collections.IDictionary]) {
            foreach ($key in $Object.Keys) {
                $hash[$key] = Sort-JsonKeys $Object[$key]
            }
        } else {
            foreach ($prop in $Object.PSObject.Properties) {
                $hash[$prop.Name] = Sort-JsonKeys $prop.Value
            }
        }
        
        # Create ordered object with sorted keys
        $sorted = [ordered]@{}
        foreach ($key in ($hash.Keys | Sort-Object)) {
            $sorted[$key] = $hash[$key]
        }
        
        return $sorted
    }
    elseif ($Object -is [Array]) {
        # Recursively sort each array element
        return @($Object | ForEach-Object { Sort-JsonKeys $_ })
    }
    else {
        # Primitive value - return as-is
        return $Object
    }
}

function Normalize-JsonFile {
    param(
        [string]$FilePath
    )
    
    try {
        Write-Host "Normalizing: $FilePath" -ForegroundColor Cyan
        
        # Read and parse JSON
        $content = Get-Content -Path $FilePath -Raw | ConvertFrom-Json
        
        # Sort keys recursively
        $sorted = Sort-JsonKeys $content
        
        # Write back with consistent formatting
        $json = $sorted | ConvertTo-Json -Depth 100 -Compress:$false
        Set-Content -Path $FilePath -Value $json -NoNewline -Encoding UTF8
        
        Write-Host "  ✓ Normalized" -ForegroundColor Green
    }
    catch {
        Write-Host "  ✗ Error: $_" -ForegroundColor Red
    }
}

# Process files
if (Test-Path $Path) {
    if ((Get-Item $Path).PSIsContainer) {
        # Directory - process all JSON files
        $files = Get-ChildItem -Path $Path -Filter "*.json" -File
        Write-Host "Processing $($files.Count) JSON files in $Path`n" -ForegroundColor Yellow
        
        foreach ($file in $files) {
            Normalize-JsonFile -FilePath $file.FullName
        }
    }
    else {
        # Single file
        Normalize-JsonFile -FilePath $Path
    }
}
else {
    Write-Error "Path not found: $Path"
}

Write-Host "`nNormalization complete!" -ForegroundColor Green
