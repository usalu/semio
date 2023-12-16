using System;
using System.Collections.Generic;
using Grasshopper.Kernel;
using Semio.Grasshopper.Goos;

namespace Semio.Grasshopper.Params;

public class ScriptParam : SemioPersistentParam<ScriptGoo>
{
    public ScriptParam() : base("Script", "S", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("895BBC91-851A-4DFC-9C83-92DFE90029E8");

    protected override GH_GetterResult Prompt_Singular(ref ScriptGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<ScriptGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class ScriptKindParam : SemioPersistentParam<ScriptKindGoo>
{
    public ScriptKindParam() : base("ScriptKind", "SK", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("DBE104DA-63FA-4C68-8D41-834DD962F1D7");

    protected override GH_GetterResult Prompt_Singular(ref ScriptKindGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<ScriptKindGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class PropertyParam : SemioPersistentParam<PropertyGoo>
{
    public PropertyParam() : base("Property", "Pp", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1B");

    protected override GH_GetterResult Prompt_Singular(ref PropertyGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<PropertyGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class TypeParam : SemioPersistentParam<TypeGoo>
{
    public TypeParam() : base("Type", "T", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");

    protected override GH_GetterResult Prompt_Singular(ref TypeGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<TypeGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class PortParam : SemioPersistentParam<PortGoo>
{
    public PortParam() : base("Port", "Po", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("371F1E80-28FF-4197-9289-E9EB35BDF999");

    protected override GH_GetterResult Prompt_Singular(ref PortGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<PortGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class PieceParam : SemioPersistentParam<PieceGoo>
{
    public PieceParam() : base("Piece", "Pc", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("76F583DC-4142-4346-B1E1-6C241AF26086");

    protected override GH_GetterResult Prompt_Singular(ref PieceGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<PieceGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class AttractionParam : SemioPersistentParam<AttractionGoo>
{
    public AttractionParam() : base("Attraction", "A", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");

    protected override GH_GetterResult Prompt_Singular(ref AttractionGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<AttractionGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class FormationParam : SemioPersistentParam<FormationGoo>
{
    public FormationParam() : base("Formation", "F", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");

    protected override GH_GetterResult Prompt_Singular(ref FormationGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<FormationGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class KitParam : SemioPersistentParam<KitGoo>
{
    public KitParam() : base("Kit", "K", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("F2F6F2F9-7F0E-4F0F-9F0C-7F6F6F6F6F6F");

    protected override GH_GetterResult Prompt_Singular(ref KitGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<KitGoo> values)
    {
        throw new NotImplementedException();
    }
}