# Refactor Plan: Semio.cs & Semio.Grasshopper.cs

## Executive Summary

This refactor eliminates the reflection-based attribute system in favor of:
1. **Plain validation** using FluentValidation (already present)
2. **UI metadata separation** - move all display concerns to Grasshopper layer
3. **Simpler core models** - remove Entity base class metadata infrastructure
4. **Direct property access** - no reflection for icons, descriptions, symbols

## Current Architecture Problems

### 1. Reflection-Based Metadata System
- `EntityAttribute` stores UI metadata (description, icon, symbol) on domain models
- Grasshopper components use `System.Attribute.GetCustomAttribute()` to retrieve metadata
- Performance overhead from reflection calls
- Tight coupling between domain and UI concerns

### 2. Entity Base Class Complexity
- `Entity<T>` base class handles serialization, cloning, diffing, AND metadata
- Mixed concerns violate separation of responsibilities
- Generic constraints propagate through entire hierarchy

### 3. Component Hierarchy Issues
- Base classes (`Goo<T>`, `Param<T>`, `Component<T>`) rely on reflection
- Icon loading: `Resources.ResourceManager.GetObject($"{typeof(TModel).Name.ToLower()}_24x24")`
- Description retrieval: `((EntityAttribute)System.Attribute.GetCustomAttribute(...))`
- TypeName/TypeDescription pulled from attributes via reflection

---

## Refactor Plan: Semio.cs

### Phase 1: Remove Attribute System

#### 1.1 Delete EntityAttribute Class
**Location:** Lines ~1487-1520 (estimated from context)

**Current:**
```csharp
[AttributeUsage(AttributeTargets.Class)]
public class EntityAttribute : System.Attribute
{
    public string Description { get; set; }
    public string Icon { get; set; }
    public string Symbol { get; set; }
}
```

**Action:** Delete entire `EntityAttribute` class and all `[Entity(...)]` decorators on model classes.

**Impact:** All entity classes (Attribute, Coord, Point, Vector, Plane, Location, Author, File, Folder, Benchmark, Quality, Tag, Concept, Interface, Prop, Model, Connector, Type, Layer, Group, Piece, Side, Connection, Stat, Design, Kit)

#### 1.2 Simplify Entity Base Class
**Location:** Entity<T> class definition

**Current:**
```csharp
public abstract class Entity<T> where T : Entity<T>, new()
{
    // Serialization
    // Cloning
    // Diffing
    // Metadata retrieval via reflection
}
```

**Remove:**
- Any methods that retrieve metadata via reflection
- Any properties that expose Description, Icon, Symbol from attributes
- Meta class integration for attribute-based metadata

**Keep:**
- Serialization methods (ToJson, FromJson)
- Cloning methods (Clone)
- Diffing methods (GetDiff, ApplyDiff, InverseDiff, MergeDiff)
- Validation methods (already using FluentValidation)

**New approach:**
```csharp
public abstract class Entity<T> where T : Entity<T>, new()
{
    // Core functionality only
    public abstract string ToJson();
    public abstract T FromJson(string json);
    public abstract T Clone();
    public abstract T GetDiff(T other);
    public abstract T ApplyDiff(T diff);
    public abstract T InverseDiff(T original, T diff);
    public abstract T MergeDiff(T diff1, T diff2);
    
    // Validation stays (FluentValidation)
    public ValidationResult Validate() => /* existing */;
}
```

#### 1.3 Remove Meta Class Reflection Logic
**Location:** Lines 6495-6610 (Meta region)

**Current:**
```csharp
public static class Meta
{
    // Uses reflection to get EntityAttribute metadata
    public static string GetDescription(Type type) { /* reflection */ }
    public static string GetIcon(Type type) { /* reflection */ }
    public static string GetSymbol(Type type) { /* reflection */ }
}
```

**Action:** Delete entire Meta class or reduce to non-reflection utilities only.

**Alternative:** If Meta is used for other purposes, keep only non-metadata methods.

### Phase 2: Strengthen Validation

#### 2.1 Expand FluentValidation Rules
**Location:** SemioValidation region (Lines 1831-2126)

**Current:** Already has validators like `AttributeValidator`, `CoordValidator`, etc.

**Enhance:**
- Ensure ALL business rules are in validators (not scattered in attributes)
- Add constraint-based validation (uniqueness, references, etc.)
- Consolidate validation logic currently split between attributes and validators

**Example:**
```csharp
public class KitValidator : AbstractValidator<Kit>
{
    public KitValidator()
    {
        // Existing rules
        RuleFor(k => k.Name)
            .NotEmpty()
            .MaximumLength(Constants.NameLengthLimit);
            
        // Add semantic rules previously implied by attributes
        RuleFor(k => k.Types)
            .Must(types => types.Select(t => t.Guid).Distinct().Count() == types.Count)
            .WithMessage("Type GUIDs must be unique within kit");
            
        // Add cross-entity validation
        RuleFor(k => k.Designs)
            .Must((kit, designs) => ValidateDesignReferences(kit, designs))
            .WithMessage("Design pieces must reference valid types");
    }
    
    private bool ValidateDesignReferences(Kit kit, List<Design> designs)
    {
        var typeGuids = kit.Types.Select(t => t.Guid).ToHashSet();
        return designs.All(d => 
            d.Pieces.All(p => typeGuids.Contains(p.Type?.Guid)));
    }
}
```

#### 2.2 Remove Validation from ToString/Display Logic
**Location:** All entity classes

**Current:** Some entities may have validation mixed with display logic

**Action:** 
- Ensure validation is ONLY in validator classes
- Remove any validation checks from ToString(), display methods, or property setters
- Use validators explicitly at domain boundaries

### Phase 3: Clean Up Entity Classes

#### 3.1 Remove Attribute Decorators
**Action:** Remove all `[Entity(Description = "...", Icon = "...", Symbol = "...")]` decorators from:
- Attribute class (Line ~2137)
- Coord class (Line ~2258)
- Point class (Line ~2281)
- Vector class (Line ~2299)
- Plane class (Line ~2340)
- Location class (Line ~2382)
- Author class (Line ~2415)
- File class (Line ~2543)
- Folder class (Line ~2652)
- Benchmark class (Line ~2768)
- QualityKind enum (Line ~2810)
- Quality class (Line ~2826)
- Tag class (Line ~2959)
- Concept class (Line ~2995)
- Interface class (Line ~3067)
- Prop class (Line ~3187)
- Model class (Line ~3224)
- Connector class (Line ~3384)
- Type class (Line ~3652)
- Layer class (Line ~4040)
- Group class (Line ~4081)
- Piece class (Line ~4120)
- Side class (Line ~4280)
- Connection class (Line ~4376)
- Stat class (Line ~4611)
- Design class (Line ~4652)
- Kit class (Line ~5633)

#### 3.2 Simplify Entity Methods
**Location:** Each entity class

**Remove:**
- Any methods that call Meta.GetDescription(), Meta.GetIcon(), etc.
- Any ToString() implementations that include metadata
- Any display-related methods that aren't core domain logic

**Keep:**
- Business logic methods
- Computation methods (e.g., Design.ComputePlanes())
- Serialization/deserialization
- Validation triggers

---

## Refactor Plan: Semio.Grasshopper.cs

### Phase 1: Create Metadata Registry

#### 1.1 Define Metadata Structure
**Location:** New region after Constants (Line ~92)

**Add:**
```csharp
#region Metadata

public class EntityMetadata
{
    public string Name { get; set; }
    public string Description { get; set; }
    public string IconResourceName { get; set; }
    public string Symbol { get; set; }
    public string Nickname { get; set; }
}

public static class EntityMetadataRegistry
{
    private static readonly Dictionary<Type, EntityMetadata> _metadata = new()
    {
        [typeof(Semio.Attribute)] = new()
        {
            Name = "Attribute",
            Description = "Key-value metadata with optional unit and definition",
            IconResourceName = "attribute_24x24",
            Symbol = "Attr",
            Nickname = "Att"
        },
        [typeof(Coord)] = new()
        {
            Name = "Coord",
            Description = "2D coordinate with U and V components",
            IconResourceName = "coord_24x24",
            Symbol = "UV",
            Nickname = "Crd"
        },
        [typeof(Point)] = new()
        {
            Name = "Point",
            Description = "3D point with X, Y, Z coordinates",
            IconResourceName = "point_24x24",
            Symbol = "Pt",
            Nickname = "Pt"
        },
        [typeof(Vector)] = new()
        {
            Name = "Vector",
            Description = "3D vector with X, Y, Z components",
            IconResourceName = "vector_24x24",
            Symbol = "Vec",
            Nickname = "Vec"
        },
        [typeof(Plane)] = new()
        {
            Name = "Plane",
            Description = "3D plane with origin and orientation",
            IconResourceName = "plane_24x24",
            Symbol = "Pln",
            Nickname = "Pln"
        },
        [typeof(Location)] = new()
        {
            Name = "Location",
            Description = "Geographic location with coordinates and attributes",
            IconResourceName = "location_24x24",
            Symbol = "Loc",
            Nickname = "Loc"
        },
        [typeof(Author)] = new()
        {
            Name = "Author",
            Description = "Kit contributor with name, email, and rank",
            IconResourceName = "author_24x24",
            Symbol = "Auth",
            Nickname = "Aut"
        },
        [typeof(Semio.File)] = new()
        {
            Name = "File",
            Description = "Kit file with path, remote URL, and metadata",
            IconResourceName = "file_24x24",
            Symbol = "File",
            Nickname = "Fil"
        },
        [typeof(Folder)] = new()
        {
            Name = "Folder",
            Description = "Kit folder with path and metadata",
            IconResourceName = "folder_24x24",
            Symbol = "Fldr",
            Nickname = "Fld"
        },
        [typeof(Benchmark)] = new()
        {
            Name = "Benchmark",
            Description = "Performance standard with name, range, and icon",
            IconResourceName = "benchmark_24x24",
            Symbol = "Bench",
            Nickname = "Bmk"
        },
        [typeof(Quality)] = new()
        {
            Name = "Quality",
            Description = "Measurement definition with formula and constraints",
            IconResourceName = "quality_24x24",
            Symbol = "Qual",
            Nickname = "Qlt"
        },
        [typeof(Tag)] = new()
        {
            Name = "Tag",
            Description = "Model categorization tag with name and icon",
            IconResourceName = "tag_24x24",
            Symbol = "Tag",
            Nickname = "Tag"
        },
        [typeof(Concept)] = new()
        {
            Name = "Concept",
            Description = "Semantic grouping with name and order",
            IconResourceName = "concept_24x24",
            Symbol = "Cncpt",
            Nickname = "Cnc"
        },
        [typeof(Semio.Interface)] = new()
        {
            Name = "Interface",
            Description = "Connector compatibility definition",
            IconResourceName = "interface_24x24",
            Symbol = "Intf",
            Nickname = "Int"
        },
        [typeof(Prop)] = new()
        {
            Name = "Prop",
            Description = "Connector property with quality reference",
            IconResourceName = "prop_24x24",
            Symbol = "Prop",
            Nickname = "Prp"
        },
        [typeof(Model)] = new()
        {
            Name = "Model",
            Description = "3D model with tags and file reference",
            IconResourceName = "model_24x24",
            Symbol = "Mdl",
            Nickname = "Mdl"
        },
        [typeof(Connector)] = new()
        {
            Name = "Connector",
            Description = "Type connection point with direction and interface",
            IconResourceName = "connector_24x24",
            Symbol = "Conn",
            Nickname = "Con"
        },
        [typeof(Semio.Type)] = new()
        {
            Name = "Type",
            Description = "Reusable component with models and connectors",
            IconResourceName = "type_24x24",
            Symbol = "Type",
            Nickname = "Typ"
        },
        [typeof(Layer)] = new()
        {
            Name = "Layer",
            Description = "Design organizational layer with visibility and color",
            IconResourceName = "layer_24x24",
            Symbol = "Lyr",
            Nickname = "Lyr"
        },
        [typeof(Group)] = new()
        {
            Name = "Group",
            Description = "Design piece grouping with name and color",
            IconResourceName = "group_24x24",
            Symbol = "Grp",
            Nickname = "Grp"
        },
        [typeof(Piece)] = new()
        {
            Name = "Piece",
            Description = "Type or design instance with placement and properties",
            IconResourceName = "piece_24x24",
            Symbol = "Pce",
            Nickname = "Pce"
        },
        [typeof(Side)] = new()
        {
            Name = "Side",
            Description = "Connection side with piece and connector reference",
            IconResourceName = "side_24x24",
            Symbol = "Side",
            Nickname = "Sid"
        },
        [typeof(Connection)] = new()
        {
            Name = "Connection",
            Description = "Link between two pieces with transform parameters",
            IconResourceName = "connection_24x24",
            Symbol = "Conn",
            Nickname = "Con"
        },
        [typeof(Stat)] = new()
        {
            Name = "Stat",
            Description = "Design performance measurement",
            IconResourceName = "stat_24x24",
            Symbol = "Stat",
            Nickname = "Sta"
        },
        [typeof(Design)] = new()
        {
            Name = "Design",
            Description = "Assembly of pieces and connections",
            IconResourceName = "design_24x24",
            Symbol = "Dsgn",
            Nickname = "Des"
        },
        [typeof(Kit)] = new()
        {
            Name = "Kit",
            Description = "Collection of types, designs, and metadata",
            IconResourceName = "kit_24x24",
            Symbol = "Kit",
            Nickname = "Kit"
        }
    };
    
    public static EntityMetadata Get(Type type)
    {
        return _metadata.TryGetValue(type, out var metadata) 
            ? metadata 
            : new EntityMetadata 
            { 
                Name = type.Name, 
                Description = type.Name,
                IconResourceName = $"{type.Name.ToLower()}_24x24",
                Symbol = type.Name,
                Nickname = type.Name.Substring(0, Math.Min(3, type.Name.Length))
            };
    }
    
    public static string GetName<T>() => Get(typeof(T)).Name;
    public static string GetDescription<T>() => Get(typeof(T)).Description;
    public static string GetIconResourceName<T>() => Get(typeof(T)).IconResourceName;
    public static string GetSymbol<T>() => Get(typeof(T)).Symbol;
    public static string GetNickname<T>() => Get(typeof(T)).Nickname;
}

#endregion Metadata
```

### Phase 2: Refactor Base Classes

#### 2.1 Simplify Goo<TEntity>
**Location:** Lines 256-303

**Current:**
```csharp
public override string TypeName => typeof(TEntity).Name;
public override string TypeDescription => ((EntityAttribute)System.Attribute.GetCustomAttribute(typeof(TEntity), typeof(EntityAttribute))).Description;
```

**Replace with:**
```csharp
public override string TypeName => EntityMetadataRegistry.GetName<TEntity>();
public override string TypeDescription => EntityMetadataRegistry.GetDescription<TEntity>();
```

**Remove:** All reflection calls to EntityAttribute

#### 2.2 Simplify Param<TGoo, TModel>
**Location:** Lines 305-316

**Current:**
```csharp
protected override Bitmap Icon => (Bitmap)(Resources.ResourceManager.GetObject($"{typeof(TModel).Name.ToLower()}_24x24") ?? throw new InvalidOperationException($"Resource {typeof(TModel).Name.ToLower()}_24x24 not found"));
```

**Replace with:**
```csharp
protected override Bitmap Icon => 
    (Bitmap)(Resources.ResourceManager.GetObject(EntityMetadataRegistry.GetIconResourceName<TModel>()) 
    ?? throw new InvalidOperationException($"Resource {EntityMetadataRegistry.GetIconResourceName<TModel>()} not found"));
```

**Or simplify further:**
```csharp
private static readonly Lazy<Bitmap> _icon = new Lazy<Bitmap>(() =>
    (Bitmap)Resources.ResourceManager.GetObject(EntityMetadataRegistry.GetIconResourceName<TModel>()) 
    ?? throw new InvalidOperationException($"Icon not found for {typeof(TModel).Name}"));
    
protected override Bitmap Icon => _icon.Value;
```

#### 2.3 Update PassthroughComponent<TParam, TGoo, TModel>
**Location:** Lines 359-428

**Current:**
```csharp
protected virtual string ModelName => typeof(TModel).Name;
protected virtual string ModelNickname => typeof(TModel).Name.Substring(0, 3);
protected virtual string ModelDescription => $"Passthrough (either construct, deconstruct or modify) a {typeof(TModel).Name.ToLower()}.";
```

**Replace with:**
```csharp
protected virtual string ModelName => EntityMetadataRegistry.GetName<TModel>();
protected virtual string ModelNickname => EntityMetadataRegistry.GetNickname<TModel>();
protected virtual string ModelDescription => $"Passthrough (either construct, deconstruct or modify) a {EntityMetadataRegistry.GetName<TModel>().ToLower()}.";
```

#### 2.4 Update IdParam<TGoo, TModel>
**Location:** Lines 434-441

**Current:**
```csharp
protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{typeof(TModel).Name.ToLower().Substring(0, typeof(TModel).Name.Length - 2)}_id_24x24");
```

**Replace with:**
```csharp
protected override Bitmap Icon => 
    (Bitmap)(Resources.ResourceManager.GetObject($"{EntityMetadataRegistry.GetIconResourceName<TModel>().Replace("_24x24", "")}_id_24x24")
    ?? throw new InvalidOperationException($"ID icon not found for {typeof(TModel).Name}"));
```

**Or add explicit ID icon mapping in registry:**
```csharp
// In EntityMetadata class
public string IdIconResourceName { get; set; }

// In registry initialization
IdIconResourceName = "attribute_id_24x24"

// In IdParam
protected override Bitmap Icon => 
    (Bitmap)Resources.ResourceManager.GetObject(EntityMetadataRegistry.Get(typeof(TModel)).IdIconResourceName);
```

### Phase 3: Update All Component Regions

#### 3.1 General Pattern for Each Region

Each entity region (Attribute, Coord, Location, Author, File, Folder, etc.) follows the same pattern. Apply these changes to ALL entity regions:

**Before:**
```csharp
// Relies on reflection to get icon
protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{typeof(TModel).Name.ToLower()}_24x24");

// Relies on EntityAttribute
public override string TypeDescription => ((EntityAttribute)...).Description;
```

**After:**
```csharp
// Uses metadata registry
protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject(EntityMetadataRegistry.GetIconResourceName<TModel>());

// Uses metadata registry
public override string TypeDescription => EntityMetadataRegistry.GetDescription<TModel>();
```

#### 3.2 Specific Regions to Update

Apply metadata registry pattern to:

1. **Attribute region** (Lines 685-955)
2. **Coord region** (Lines 960-1037)
3. **Location region** (Lines 1042-1122)
4. **Author region** (Lines 1127-1242)
5. **File region** (Lines 1248-1475)
6. **Folder region** (Lines 1480-1731)
7. **Benchmark region** (Lines 1736-1787)
8. **QualityKind region** (Lines 1792-1802)
9. **Quality region** (Lines 1807-2041)
10. **Tag region** (Lines 2046-2177)
11. **Prop region** (Lines 2182-2239)
12. **Model region** (Lines 2244-2505)
13. **Connector region** (Lines 2510-2834)
14. **Concept region** (Lines 2839-2970)
15. **Interface region** (Lines 2975-3111)
16. **Type region** (Lines 3116-3468)
17. **Layer region** (Lines 3473-3577)
18. **Group region** (Lines 3582-3679)
19. **Piece region** (Lines 3684-4003)
20. **Side region** (Lines 4008-4146)
21. **Connection region** (Lines 4151-4493)
22. **Stat region** (Lines 4498-4600)
23. **Design region** (Lines 4605-4931)
24. **Kit region** (Lines 4936-5242)

### Phase 4: Remove Reflection from Utility

#### 4.1 Update Utility Class
**Location:** Lines 97-224

**Review:** Check if Utility class has any reflection-based metadata retrieval

**Action:** Replace any reflection calls with direct metadata registry calls

---

## Implementation Strategy

### Step 1: Prepare Metadata Registry (Grasshopper)
1. Create `EntityMetadataRegistry` class
2. Populate all 26 entity metadata entries
3. Add helper methods for common operations
4. Test metadata retrieval with unit tests

### Step 2: Update Base Classes (Grasshopper)
1. Update `Goo<TEntity>` to use registry
2. Update `Param<TGoo, TModel>` to use registry
3. Update `PassthroughComponent<TParam, TGoo, TModel>` to use registry
4. Update `IdParam` and `IdComponent` to use registry
5. Update `DiffGoo` and `DiffParam` to use registry

### Step 3: Update All Component Regions (Grasshopper)
1. Go through each of 26 entity regions
2. Replace reflection calls with registry calls
3. Verify icon loading still works
4. Verify tooltips/descriptions still work

### Step 4: Simplify Core Models (Semio.cs)
1. Remove `EntityAttribute` class definition
2. Remove all `[Entity(...)]` decorators from entity classes
3. Simplify `Entity<T>` base class (remove metadata methods)
4. Delete or simplify `Meta` class
5. Ensure validators are complete and sufficient

### Step 5: Testing & Validation
1. **Unit tests:** Verify metadata registry returns correct values
2. **Integration tests:** Load components in Grasshopper, verify icons/descriptions appear
3. **Validation tests:** Ensure FluentValidation catches all constraint violations
4. **Performance tests:** Measure reflection elimination impact (should be faster)

### Step 6: Documentation Updates
1. Update AGENTS.md: Document new metadata registry pattern
2. Update README.md: Explain separation of domain/UI concerns
3. Add inline comments explaining metadata registry usage
4. Document validation approach (FluentValidation only)

---

## Benefits of This Refactor

### 1. Performance
- **Eliminate reflection overhead:** No more `System.Attribute.GetCustomAttribute()` calls
- **Compile-time safety:** Metadata registry is strongly typed
- **Lazy loading:** Icons loaded once per type, not per component instance

### 2. Maintainability
- **Separation of concerns:** Domain models have no UI metadata
- **Single source of truth:** All UI metadata in one registry class
- **Easier to extend:** Add new metadata fields without changing entity classes

### 3. Clarity
- **Explicit over implicit:** Metadata registry is obvious and searchable
- **No magic:** No reflection, no attribute scanning, no runtime discovery
- **Type safety:** Compiler catches missing metadata at compile time

### 4. Testability
- **Mock friendly:** Registry can be swapped for testing
- **Validation isolated:** Validators are pure functions, easy to test
- **Domain logic pure:** Entity classes are data structures with business logic only

---

## Migration Checklist

### Semio.cs Changes
- [ ] Delete `EntityAttribute` class
- [ ] Remove all `[Entity(...)]` decorators from 26+ entity classes
- [ ] Simplify `Entity<T>` base class (remove metadata methods)
- [ ] Delete or reduce `Meta` class (remove reflection methods)
- [ ] Verify all validators are complete (no business logic in attributes)
- [ ] Run validation tests to ensure no regressions

### Semio.Grasshopper.cs Changes
- [ ] Create `EntityMetadata` class
- [ ] Create `EntityMetadataRegistry` with all 26 entities
- [ ] Update `Goo<TEntity>` base class
- [ ] Update `Param<TGoo, TModel>` base class
- [ ] Update `PassthroughComponent` base class
- [ ] Update `IdParam` and `IdComponent` base classes
- [ ] Update `DiffGoo` and `DiffParam` base classes
- [ ] Update EnumGoo/EnumParam base classes (if they use reflection)
- [ ] Update all 26 entity regions to use registry
- [ ] Remove any reflection calls from `Utility` class
- [ ] Test all components load in Grasshopper
- [ ] Verify icons display correctly
- [ ] Verify tooltips/descriptions display correctly

### Testing
- [ ] Unit test metadata registry completeness
- [ ] Unit test validator coverage
- [ ] Integration test Grasshopper component loading
- [ ] Integration test icon resource loading
- [ ] Performance benchmark (before/after reflection removal)
- [ ] Validation test suite (ensure no constraint gaps)

### Documentation
- [ ] Update AGENTS.md with new patterns
- [ ] Update README.md with architecture changes
- [ ] Add inline comments in metadata registry
- [ ] Document validation approach
- [ ] Update any developer guides

---

## Risk Assessment

### Low Risk
- **Metadata registry creation:** Straightforward data structure
- **Base class updates:** Isolated changes with clear patterns
- **Documentation:** No functional impact

### Medium Risk
- **Icon resource name mapping:** Must match existing resource names exactly
- **Nickname generation:** Must match existing Grasshopper component nicknames
- **Description text:** Must maintain consistency with existing tooltips

### High Risk
- **Entity attribute removal:** Affects every entity class, must be thorough
- **Validation completeness:** Must ensure no business rules are lost
- **Component region updates:** 26 regions to update, risk of inconsistency

### Mitigation Strategies
1. **Automated testing:** Write tests before removing attributes
2. **Incremental migration:** Update one entity at a time, test each
3. **Validation inventory:** Document all existing attribute-based constraints
4. **Backward compatibility:** Keep old attributes temporarily with obsolete warnings
5. **Code review:** Multiple reviewers for EntityAttribute removal
6. **Rollback plan:** Version control with clear commit boundaries

---

## Timeline Estimate

- **Phase 1 (Metadata Registry):** 2-4 hours
- **Phase 2 (Base Classes):** 3-5 hours
- **Phase 3 (Component Regions):** 8-12 hours (26 regions × ~20-30 min each)
- **Phase 4 (Core Model Cleanup):** 2-4 hours
- **Phase 5 (Testing):** 4-6 hours
- **Phase 6 (Documentation):** 2-3 hours

**Total:** 21-34 hours (3-4 days of focused work)

---

## Open Questions

1. **Icon naming consistency:** Do all 26 entities have matching resource names in Resources.resx?
2. **Meta class usage:** Is Meta class used only for EntityAttribute, or are there other reflection-based utilities?
3. **Validation coverage:** Are there any business rules currently enforced via attributes that aren't in validators?
4. **Enum handling:** How should enum metadata (e.g., QualityKind) be handled in the registry?
5. **Extensibility:** Should the metadata registry support plugin/extension scenarios?

---

## Conclusion

This refactor eliminates reflection-based metadata retrieval in favor of explicit registry-based metadata management. The core domain models (Semio.cs) become pure data structures with validation, while the UI layer (Semio.Grasshopper.cs) owns all display concerns. This improves performance, maintainability, testability, and adheres to separation of concerns principles.

The refactor is mechanical and low-risk if executed incrementally with thorough testing. The metadata registry provides a single source of truth for all UI-related entity information, making future changes easier and more predictable.
