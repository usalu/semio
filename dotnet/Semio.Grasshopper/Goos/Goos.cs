using Grasshopper.Kernel.Types;

namespace Semio.Grasshopper.Goos;

public class ScriptGoo : SemioGoo<Script>
{
    public ScriptGoo()
    {
        Value = new Script();
    }

    public ScriptGoo(Script script)
    {
        Value = script;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Script";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new ScriptGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}

public class ScriptKindGoo : SemioGoo<ScriptKind>
{
    public ScriptKindGoo()
    {
        Value = new ScriptKind();
    }

    public ScriptKindGoo(ScriptKind scriptKind)
    {
        Value = scriptKind;
    }

    public override bool IsValid { get; }
    public override string TypeName => "ScriptKind";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new ScriptKindGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}

public class PropertyGoo : SemioGoo<Property>
{
    public PropertyGoo()
    {
        Value = new Property();
    }

    public PropertyGoo(Property property)
    {
        Value = property;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Property";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new PropertyGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}

public class TypeGoo : SemioGoo<Type>
{
    public TypeGoo()
    {
        Value = new Type();
    }

    public TypeGoo(Type type)
    {
        Value = type;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Type";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new TypeGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}

public class PortGoo : SemioGoo<Port>
{
    public PortGoo()
    {
        Value = new Port();
    }

    public PortGoo(Port port)
    {
        Value = port;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Port";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new PortGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}

public class PieceGoo : SemioGoo<Piece>
{
    public PieceGoo()
    {
        Value = new Piece();
    }

    public PieceGoo(Piece piece)
    {
        Value = piece;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Piece";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new PieceGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}

public class AttractionGoo : SemioGoo<Attraction>
{
    public AttractionGoo()
    {
        Value = new Attraction();
    }

    public AttractionGoo(Attraction attraction)
    {
        Value = attraction;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Attraction";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new AttractionGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}

public class FormationGoo : SemioGoo<Formation>
{
    public FormationGoo()
    {
        Value = new Formation();
    }

    public FormationGoo(Formation formation)
    {
        Value = formation;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Formation";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new FormationGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}
public class KitGoo : SemioGoo<Kit>
{
    public KitGoo()
    {
        Value = new Kit();
    }

    public KitGoo(Kit kit)
    {
        Value = kit;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Kit";
    public override string TypeDescription { get; }


    public override IGH_Goo Duplicate()
    {
        return new KitGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }

}