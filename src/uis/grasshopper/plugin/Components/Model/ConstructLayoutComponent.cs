using System;
using System.Collections.Generic;
using System.Linq;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;

namespace Semio.UI.Grasshopper
{
    public class ConstructLayoutComponent : GH_Component
    {
        public ConstructLayoutComponent()
          : base("Construct Layout", "Layout", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            var fields = Layout.Descriptor.Fields.InFieldNumberOrder();
            FieldDescriptor field;
            //Sadly field.Declaration.LeadingComments is null at runtime :(
            field = fields[0];
            pManager.AddParameter(new SobjectParam(),field.Name, "S", "", GH_ParamAccess.list);
            field = fields[1];
            pManager.AddParameter(new AttractionParam(), field.Name, "A", "", GH_ParamAccess.list);
            pManager[1].Optional = true;
            field = fields[2];
            pManager.AddParameter(new SobjectParam(),field.Name, "RI", "", GH_ParamAccess.item);
            pManager[2].Optional = true;
            field = fields[3];
            pManager.AddParameter(new LayoutStrategyParam(), field.Name, "S","",GH_ParamAccess.item);
            pManager[3].Optional = true;
            field = fields[4];
            pManager.AddParameter(new AttractionTreeParam(), field.Name, "AT", "", GH_ParamAccess.list);
            pManager[4].Optional = true;
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new LayoutParam());
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            var sobjects = new List<SobjectGoo>();
            if (!DA.GetDataList(0, sobjects)) return;

            var attractions = new List<AttractionGoo>();
            DA.GetDataList(1, attractions);

            Sobject rootSobject = new();
            DA.GetData(2, ref rootSobject);

            LayoutStrategyGoo strategy = new();
            DA.GetData(3, ref strategy);

            var attractionTrees = new List<AttractionTreeGoo>();
            DA.GetDataList(4, attractionTrees);

            Layout layout = new Layout()
            {
               RootSobjectId = rootSobject.Id,
               Stragegy = strategy.Value
            };

            layout.Sobjects.AddRange(sobjects.Select(x => x.Value));
            layout.Attractions.AddRange(attractions.Select(x => x.Value));
            layout.AttractionTrees.AddRange(attractionTrees.Select(x => x.Value));

            DA.SetData(0, new LayoutGoo(layout));
        }
        public override Guid ComponentGuid => new("E866DD2D-3A02-4540-8A05-F9C0387F7503");
    }
}