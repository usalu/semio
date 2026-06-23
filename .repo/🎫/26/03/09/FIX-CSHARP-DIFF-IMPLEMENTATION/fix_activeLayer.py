filepath = '/workspaces/semio/compose/net/Compose/Compose.cs'
with open(filepath, 'r') as f:
    content = f.read()

fixes = 0

# Fix 1: DesignDiff._activeLayer field type
old = '    private string? _activeLayer;'
new = '    private LayerId? _activeLayer;'
if old in content:
    content = content.replace(old, new, 1)
    fixes += 1
    print("Fixed DesignDiff._activeLayer field type")

# Fix 2: DesignDiff.ActiveLayer property type
old = '    public string? ActiveLayer { get => _activeLayer; set { _activeLayer = value; _setProperties.Add("ActiveLayer"); } }'
new = '    public LayerId? ActiveLayer { get => _activeLayer; set { _activeLayer = value; _setProperties.Add("ActiveLayer"); } }'
if old in content:
    content = content.replace(old, new, 1)
    fixes += 1
    print("Fixed DesignDiff.ActiveLayer property type")

# Fix 3: Design.ActiveLayer property type
old = '    public string? ActiveLayer { get; set; }'
new = '    public LayerId? ActiveLayer { get; set; }'
if old in content:
    content = content.replace(old, new, 1)
    fixes += 1
    print("Fixed Design.ActiveLayer property type")

# Fix 4: SQL reader - wrap string in LayerId
old = 'ActiveLayer = reader.IsDBNull(9) ? null : reader.GetString(9),'
new = 'ActiveLayer = reader.IsDBNull(9) ? null : new LayerId { Guid = reader.GetString(9) },'
if old in content:
    content = content.replace(old, new, 1)
    fixes += 1
    print("Fixed SQL reader ActiveLayer")

with open(filepath, 'w') as f:
    f.write(content)

print(f"\nTotal fixes: {fixes}")
