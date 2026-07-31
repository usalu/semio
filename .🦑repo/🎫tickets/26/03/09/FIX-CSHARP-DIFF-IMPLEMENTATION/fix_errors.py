import re

filepath = '/workspaces/semio/compose/net/Compose/Compose.cs'
with open(filepath, 'r') as f:
    lines = f.readlines()

fixes = 0

# L3104: ModelDiff.MergeDiff: other.Attributes.Any() -> other.Attributes != null
# "Attributes = other.Attributes.Any() ? other.Attributes : Attributes"
line = lines[3103]  # 0-indexed
if 'other.Attributes.Any()' in line:
    lines[3103] = line.replace('other.Attributes.Any()', 'other.Attributes != null')
    fixes += 1
    print(f"Fixed L3104: ModelDiff.MergeDiff .Any() -> != null")

# L3138: Model(ModelDiff diff) implicit: Attributes = diff.Attributes -> Attributes = diff.Attributes?.Added ?? new()
line = lines[3137]
if 'Attributes = diff.Attributes }' in line:
    lines[3137] = line.replace('Attributes = diff.Attributes }', 'Attributes = diff.Attributes?.Added ?? new() }')
    fixes += 1
    print(f"Fixed L3138: Model implicit operator")

# L3149: Model.ApplyDiff: diff.Attributes?.Any() == true ? diff.Attributes : model.Attributes -> AttributesDiff.Apply pattern
line = lines[3148]
if "diff.Attributes?.Any() == true ? diff.Attributes : model.Attributes" in line:
    lines[3148] = line.replace(
        "diff.Attributes?.Any() == true ? diff.Attributes : model.Attributes",
        "diff.Attributes is not null ? AttributesDiff.Apply(model.Attributes, diff.Attributes) : model.Attributes"
    )
    fixes += 1
    print(f"Fixed L3149: Model.ApplyDiff")

# L3175: Model.InverseDiff: appliedDiff.Attributes.Any() -> appliedDiff.Attributes != null
line = lines[3174]
if 'appliedDiff.Attributes.Any()' in line:
    lines[3174] = line.replace(
        'appliedDiff.Attributes.Any() ? model.Attributes : new List<Attribute>()',
        'appliedDiff.Attributes != null ? model.Attributes : null'
    )
    fixes += 1
    print(f"Fixed L3175: Model.InverseDiff")

# L3370: Connector.InverseDiff: appliedDiff.Attributes?.Any() == true -> appliedDiff.Attributes != null
line = lines[3369]
if "appliedDiff.Attributes?.Any() == true" in line:
    lines[3369] = line.replace(
        "appliedDiff.Attributes?.Any() == true ? connector.Attributes : new List<Attribute>()",
        "appliedDiff.Attributes != null ? connector.Attributes : null"
    )
    fixes += 1
    print(f"Fixed L3370: Connector.InverseDiff")

# L3587: TypeDiff.MergeDiff: other.Attributes.Any() -> other.Attributes != null
line = lines[3586]
if 'other.Attributes.Any()' in line:
    lines[3586] = line.replace(
        'other.Attributes is not null && other.Attributes.Any() ? other.Attributes : Attributes',
        'other.Attributes is not null ? other.Attributes.MergeDiff(Attributes ?? new AttributesDiff()) : Attributes'
    )
    fixes += 1
    print(f"Fixed L3587: TypeDiff.MergeDiff")

# L3654: Type(TypeDiff diff) implicit: Attributes = diff.Attributes ?? new() -> diff.Attributes?.Added ?? new()
line = lines[3653]
if 'Attributes = diff.Attributes ?? new(),' in line:
    lines[3653] = line.replace(
        'Attributes = diff.Attributes ?? new(),',
        'Attributes = diff.Attributes?.Added ?? new(),'
    )
    fixes += 1
    print(f"Fixed L3654: Type implicit operator")

# L3759: Type.InverseDiff: appliedDiff.Attributes.Any() -> appliedDiff.Attributes != null
line = lines[3758]
if 'appliedDiff.Attributes.Any()' in line:
    lines[3758] = line.replace(
        'appliedDiff.Attributes is not null && appliedDiff.Attributes.Any() ? type.Attributes : null',
        'appliedDiff.Attributes is not null ? type.Attributes : null'
    )
    fixes += 1
    print(f"Fixed L3759: Type.InverseDiff")

# L4091: Piece(PieceDiff diff) implicit: Attributes = diff.Attributes ?? new() -> diff.Attributes?.Added ?? new()
line = lines[4090]
if 'Attributes = diff.Attributes ?? new()' in line:
    lines[4090] = line.replace(
        'Attributes = diff.Attributes ?? new()',
        'Attributes = diff.Attributes?.Added ?? new()'
    )
    fixes += 1
    print(f"Fixed L4091: Piece implicit operator")

# L4110: Piece.ApplyDiff: diff.Attributes ?? piece.Attributes -> AttributesDiff.Apply
line = lines[4109]
if 'diff.Attributes ?? piece.Attributes' in line:
    lines[4109] = line.replace(
        'Attributes = diff.Attributes ?? piece.Attributes',
        'Attributes = diff.Attributes is not null ? AttributesDiff.Apply(piece.Attributes, diff.Attributes) : piece.Attributes'
    )
    fixes += 1
    print(f"Fixed L4110: Piece.ApplyDiff")

# L4369: Connection(ConnectionDiff diff) implicit: Attributes = diff.Attributes ?? new() -> diff.Attributes?.Added ?? new()
line = lines[4368]
if 'Attributes = diff.Attributes ?? new()' in line:
    lines[4368] = line.replace(
        'Attributes = diff.Attributes ?? new()',
        'Attributes = diff.Attributes?.Added ?? new()'
    )
    fixes += 1
    print(f"Fixed L4369: Connection implicit operator")

# L4386: Connection.ApplyDiff: diff.Attributes ?? connection.Attributes -> AttributesDiff.Apply
line = lines[4385]
if 'diff.Attributes ?? connection.Attributes' in line:
    lines[4385] = line.replace(
        'Attributes = diff.Attributes ?? connection.Attributes',
        'Attributes = diff.Attributes is not null ? AttributesDiff.Apply(connection.Attributes, diff.Attributes) : connection.Attributes'
    )
    fixes += 1
    print(f"Fixed L4386: Connection.ApplyDiff")

with open(filepath, 'w') as f:
    f.writelines(lines)

print(f"\nTotal fixes: {fixes}/12")
