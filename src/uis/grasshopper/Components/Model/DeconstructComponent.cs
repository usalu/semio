//TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;

namespace Semio.UI.Grasshopper.Components.Model
{
    public abstract class DeconstructComponent : GH_Component
    {

        public DeconstructComponent(string name, string nickname, string description, string category, string subCategory)
            : base(name, nickname, description, category, subCategory)
        {
        }
        public override GH_Exposure Exposure => GH_Exposure.tertiary;
    }
}