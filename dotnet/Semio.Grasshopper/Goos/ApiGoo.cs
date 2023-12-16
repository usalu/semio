using Grasshopper.Kernel.Types;

namespace Semio.Grasshopper.Goos;

public class ApiGoo : SemioGoo<Api>
{
    public ApiGoo()
    {
        Value = new Api();
    }

    public ApiGoo(Api api)
    {
        Value = api;
    }

    public override bool IsValid { get; }
    public override string TypeName { get; }
    public override string TypeDescription { get; }


    public override IGH_Goo Duplicate()
    {
        return new ApiGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
    }
}