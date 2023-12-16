using System;
using System.Collections.Generic;
using Grasshopper.Kernel;
using Semio.Grasshopper.Goos;

namespace Semio.Grasshopper.Params;

public class ApiParam : SemioPersistentParam<ApiGoo>
{
    public ApiParam() : base("API", "API", "", "semio2", "Params")
    {
    }

    public override Guid ComponentGuid => new("75BD15AB-FBE3-440A-B687-EC69B073E613");

    protected override GH_GetterResult Prompt_Singular(ref ApiGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<ApiGoo> values)
    {
        throw new NotImplementedException();
    }
}