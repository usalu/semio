using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using GH_IO.Serialization;
using System.Security.Policy;
using Google.Protobuf.Collections;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Assembler.V1;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructAssemblyComponent : ConstructComponent
    {
        public ConstructAssemblyComponent() : base("Construct Assembly", "Assembly", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new ConnectionParam(),"Connection", "A", "", GH_ParamAccess.item);
            pManager.AddParameter(new AssemblyParam(), "Childrean", "C", "", GH_ParamAccess.list);
            pManager[1].Optional=true;
        }
        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new AssemblyParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            //string connectionId = "";
            //if (!DA.GetData(0, ref connectionId)) return;

            ConnectionGoo connection = new();
            if (!DA.GetData(0, ref connection)) return;

            var children = new List<AssemblyGoo>();
            DA.GetDataList(1, children);

            Assembly assembly = new Assembly()
            {
                ConnectionId = connection.Value.Id,
            };
            assembly.Children.AddRange(children.Select(x=>x.Value));
            DA.SetData(0, new AssemblyGoo(assembly));
        }
        public override Guid ComponentGuid => new("712F6DA6-F74D-449F-83CE-956255450413");
        protected override Bitmap Icon => Resources.icon_construct_connectiontree;
    }
}