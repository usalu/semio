using System;
using Grasshopper.Kernel.Types;
using Objects.Converter.Rhino;
using Speckle.Core.Api;
using Speckle.Core.Models;

namespace Semio.Grasshopper;

public class SpeckleConverter
{
    public static string ToString(IGH_Goo goo)
    {
        Base @base;
        var item = ((dynamic)goo).Value;
        if (item is null) return "";
        switch (goo.GetType().Name)
        {
            case "GH_SpeckleBase":
                @base = (Base)item;
                break;
            default:
                var converter = new ConverterRhinoGh();
                @base = (Base)converter.ConvertToSpeckle(item);
                if (@base is null)
                    throw new ArgumentException($"The type {item.GetType()} can't be converted.");
                break;
        }

        return Operations.Serialize(@base);
    }
}