param(
    [string]$AssetsDir = "assets\semio"
)

Write-Host "Regenerating flat designs in: $AssetsDir" -ForegroundColor Cyan

# Load kit
$kitPath = Join-Path $AssetsDir "kit_metabolism.json"
$kit = Get-Content -Path $kitPath -Raw | ConvertFrom-Json

Write-Host "Loaded kit with $($kit.designs.Count) designs" -ForegroundColor White

# Find designs that need flattening (non-flat designs)
$designsToFlatten = @(
    "Nakagin Capsule Tower",
    "Slanted",
    "Twisted",
    "Dancing",
    "Capsule Dream"
)

foreach ($designName in $designsToFlatten) {
    Write-Host "`nProcessing design: $designName" -ForegroundColor Yellow
    
    # Find design in kit
    $design = $kit.designs | Where-Object { $_.name -eq $designName } | Select-Object -First 1
    
    if (-not $design) {
        Write-Warning "Design '$designName' not found in kit"
        continue
    }
    
    Write-Host "  Design GUID: $($design.guid)" -ForegroundColor Gray
    Write-Host "  This script generates flat designs in TypeScript - run via npm script" -ForegroundColor Cyan
}

Write-Host "`nTo regenerate flat designs, we need to:"  -ForegroundColor Cyan
Write-Host "  1. Call flattenDesign() from TypeScript" -ForegroundColor White
Write-Host "  2. Apply the diff to get the flat design" -ForegroundColor White  
Write-Host "  3. Save each as design_*_flat.json" -ForegroundColor White
Write-Host "`nCreating TypeScript script..." -ForegroundColor Cyan

$tsScript = @"
import { writeFileSync } from 'fs';
import { MetabolismKit } from '@semio/assets';
import { flattenDesign, applyDesignDiff, Kit } from './semio';

const kit = MetabolismKit as unknown as Kit;

const designsToFlatten = [
    { name: 'Nakagin Capsule Tower', outputFile: 'design_nakagin-capsule-tower_flat.json' },
    { name: 'Slanted', outputFile: 'design_nakagin-capsule-tower_slanted_flat.json' },
    { name: 'Twisted', outputFile: 'design_nakagin-capsule-tower_twisted_flat.json' },
    { name: 'Dancing', outputFile: 'design_nakagin-capsule-tower_dancing_flat.json' },
    { name: 'Capsule Dream', outputFile: 'design_capsule-dream_flat.json' }
];

for (const { name, outputFile } of designsToFlatten) {
    console.log(\`Processing: \${name}\`);
    const design = kit.designs?.find((d) => d.name === name);
    if (!design) {
        console.error(\`  Design '\${name}' not found\`);
        continue;
    }
    
    const flatDiff = flattenDesign(kit, design.guid);
    const flatDesign = applyDesignDiff(design, flatDiff);
    
    const outputPath = \`../../assets/semio/\${outputFile}\`;
    writeFileSync(outputPath, JSON.stringify(flatDesign, null, 2), 'utf-8');
    console.log(\`  Saved to: \${outputPath}\`);
    console.log(\`  Pieces: \${flatDesign.pieces?.length}\`);
}

console.log('\\nAll flat designs regenerated!');
"@

$tsScriptPath = "js\js\regenerate-flats.ts"
Set-Content -Path $tsScriptPath -Value $tsScript -Encoding UTF8

Write-Host "`nCreated: $tsScriptPath" -ForegroundColor Green
Write-Host "Run with: cd js/js && npx tsx regenerate-flats.ts" -ForegroundColor Cyan
